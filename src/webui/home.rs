// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Email: hex0x0000@protonmail.com

use std::sync::{LazyLock, OnceLock};

use crate::{config, plugins, utils, web_file};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use tcloud_library::serde_json;
use tokio::process::Command;

static DEFAULT_HOMEPAGE: LazyLock<PageData> = LazyLock::new(|| PageData {
    only_once: None,
    html: web_file!("homepage.html").into(),
    js: None,
    css: None,
});

static HOMEPAGE: OnceLock<PageData> = OnceLock::new();

pub static NAVBAR_ADMIN: LazyLock<Markup> = LazyLock::new(|| {
    html!(
        div id="navbar" {
            div id="logobox" {
                a href=(utils::make_url("/ui")) {
                    img id="logo" src="";
                }
            }
            div class="navelem" {
                a href=(utils::make_url("/ui/settings")) { "Settings" }
            }
            div class="navelem" {
                a href=(utils::make_url("/ui/users")) { "Users" }
            }
            @for plugin in plugins::list() {
                div class="navelem" {
                    a href=(utils::make_url(&format!("/ui/p/{}", plugin.name))) { (plugin.name) }
                }
            }
        }
    )
});

pub static NAVBAR_USER: LazyLock<Markup> = LazyLock::new(|| {
    html!(
        div id="navbar" {
            div id="logobox" {
                a href=(utils::make_url(&format!("/ui"))) {
                    img id="logo" src="";
                }
            }
            @for plugin in plugins::list() {
                @if !plugin.admin_only {
                    div class="navelem" {
                        a href=(utils::make_url(&format!("/ui/p/{}", plugin.name))) { (plugin.name) }
                    }
                }
            }
        }
    )
});

#[derive(Deserialize, Clone)]
struct PageData {
    only_once: Option<bool>,
    html: String,
    js: Option<String>,
    css: Option<String>,
}

async fn get(username: String, is_admin: bool) -> PageData {
    if let Some(page) = HOMEPAGE.get() {
        page.to_owned()
    } else {
        if let Some(script) = config!(homepage_script) {
            if let Some(page) = Command::new(script)
                .args(&[username, format!("{}", is_admin)])
                .output()
                .await
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        String::from_utf8(o.stdout).ok()
                    } else {
                        None
                    }
                })
                .and_then(|s| serde_json::from_str::<PageData>(&s).ok())
            {
                if page.only_once.unwrap_or(false) {
                    HOMEPAGE.get_or_init(|| page.clone());
                }
                page
            } else {
                log::error!("Failed to get welcome page from script. Using default page.");
                DEFAULT_HOMEPAGE.to_owned()
            }
        } else {
            DEFAULT_HOMEPAGE.to_owned()
        }
    }
}

pub async fn page(username: String, is_admin: bool) -> String {
    let page = get(username, is_admin).await;
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                title { "Home Page" }
                meta name="application-name" content=(config!(server_name));
                meta charset="utf-8";
                meta name="tcloud-prefix" content=(config!(url_prefix));
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" type="image/x-icon" href=(utils::make_url("/static/favicon.ico"));
                script type="text/javascript" {
                    (web_file!("global.js"))
                    (web_file!("navbar.js"))
                    (PreEscaped(page.js.unwrap_or("".into())))
                }
                style {
                    (web_file!("global.css"))
                    (web_file!("navbar.css"))
                    (PreEscaped(page.css.unwrap_or("".into())))
                }
            }
            body {
                @if is_admin {
                    (*NAVBAR_ADMIN)
                } @else {
                    (*NAVBAR_USER)
                }
                (PreEscaped(page.html))
            }
        }
    }
    .into()
}
