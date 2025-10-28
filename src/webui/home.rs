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

use crate::{config, plugins, unescaped_webfile, utils, webfile};
use common_library::serde_json;
use maud::{DOCTYPE, Markup, PreEscaped, html};
use serde::Deserialize;
use tokio::process::Command;

static HOMEPAGE: OnceLock<PageData> = OnceLock::new();

static NAVBAR_ADMIN: LazyLock<Markup> = LazyLock::new(|| {
    html!(
        div id="navbar" {
            div id="logobox" {
                img id="logo" src="";
            }
            div class="navelem hidden" {
                a href=(utils::make_url("/ui")) { "Home" }
            }
            div class="navelem hidden" {
                a href=(utils::make_url("/ui/settings")) { "Settings" }
            }
            div class="navelem hidden" {
                a href=(utils::make_url("/ui/users")) { "Users" }
            }
            @for plugin in plugins::list() {
                div class="navelem" {
                    a href=(utils::make_url(&format!("/ui/p/{}", plugin.name))) title=(plugin.description) {
                        (plugin.name)
                    }
                }
            }
        }
    )
});

static NAVBAR_USER: LazyLock<Markup> = LazyLock::new(|| {
    html!(
        div id="navbar" {
            div id="logobox" {
                img id="logo" src="";
            }
            div class="navelem hidden" {
                a href=(utils::make_url("/ui")) { "Home" }
            }
            div class="navelem hidden" {
                a href=(utils::make_url("/ui/settings")) { "Settings" }
            }
            @for plugin in plugins::list() {
                @if !plugin.admin_only {
                    div class="navelem" {
                        a href=(utils::make_url(&format!("/ui/p/{}", plugin.name))) title=(plugin.description) {
                            (plugin.name)
                        }
                    }
                }
            }
        }
    )
});

pub fn header(is_admin: bool) -> Markup {
    html! (
        header {
            @if is_admin {
                (*NAVBAR_ADMIN)
            } @else {
                (*NAVBAR_USER)
            }
        }
    )
}

#[derive(Deserialize, Clone)]
struct PageData {
    only_once: Option<bool>,
    html: String,
    js: Option<String>,
    css: Option<String>,
}

impl Default for PageData {
    fn default() -> Self {
        Self {
            only_once: None,
            html: unescaped_webfile!("homepage.html").into(),
            js: Some(unescaped_webfile!("homepage.js").into()),
            css: None,
        }
    }
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
                PageData::default()
            }
        } else {
            HOMEPAGE.get_or_init(|| PageData::default()).to_owned()
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
                meta charset="UTF-8";
                meta name="tcloud-prefix" content=(config!(url_prefix));
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" type="image/x-icon" href=(utils::make_url("/static/favicon.ico"));
                script type="text/javascript" {
                    (webfile!("global.js"))
                    (webfile!("navbar.js"))
                    (PreEscaped(page.js.unwrap_or("".into())))
                }
                style {
                    (webfile!("global.css"))
                    (webfile!("navbar.css"))
                    (PreEscaped(page.css.unwrap_or("".into())))
                }
            }
            body {
                (header(is_admin))
                (PreEscaped(page.html))
            }
        }
    }
    .into()
}
