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

use crate::{config, utils, web_file};
use maud::{html, PreEscaped, DOCTYPE};
use serde::Deserialize;
use tcloud_library::serde_json;
use tokio::process::Command;

static DEFAULT_WELCOME_PAGE: LazyLock<PageData> = LazyLock::new(|| PageData {
    only_once: None,
    html: web_file!("welcome.html").into(),
    js: None,
    css: None,
});
static WELCOME_PAGE: OnceLock<PageData> = OnceLock::new();

#[derive(Deserialize, Clone)]
pub struct PageData {
    pub only_once: Option<bool>,
    pub html: String,
    pub js: Option<String>,
    pub css: Option<String>,
}

pub async fn get(username: String, is_admin: bool) -> PageData {
    if let Some(page) = WELCOME_PAGE.get() {
        page.clone()
    } else {
        if let Some(script) = config!(welcome_page_script) {
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
                    WELCOME_PAGE.get_or_init(|| page.clone());
                }
                page
            } else {
                log::error!("Failed to get welcome page from script. Using default page.");
                DEFAULT_WELCOME_PAGE.to_owned()
            }
        } else {
            DEFAULT_WELCOME_PAGE.to_owned()
        }
    }
}

pub async fn page(username: String, is_admin: bool) -> String {
    let page = get(username, is_admin).await;
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                title { "Main Page" }
                meta name="application-name" content=(config!(server_name));
                meta charset="utf-8";
                meta name="tcloud-prefix" content=(config!(url_prefix));
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" type="image/x-icon" href=(utils::make_url("/static/favicon.ico"));
                script type="text/javascript" { (web_file!("global.js")) (PreEscaped(page.js.unwrap_or("".into()))) }
                style { (web_file!("global.css")) (PreEscaped(page.css.unwrap_or("".into()))) }
            }
            body {
                (PreEscaped(page.html))
            }
        }
    }
    .into()
}
