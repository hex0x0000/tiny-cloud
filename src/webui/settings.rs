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

use crate::{config, utils, webfile};
use maud::{html, PreEscaped, DOCTYPE};

pub fn page(username: String, is_admin: bool) -> String {
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                title { "Login Page" }
                meta name="application-name" content=(config!(server_name));
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="tcloud-prefix" content=(config!(url_prefix));
                link rel="icon" type="image/x-icon" href=(utils::make_url("/static/favicon.ico"));
                script type="text/javascript" { (webfile!("global.js")) (webfile!("settings.js")) }
                style { (webfile!("global.css")) }
            }
            body {
                div id="titlebar" {
                    h3 id="home" { a href=(utils::make_url("/ui")) { "Home" } }
                    h1 id="title" { "Settings" }
                }
                div id="tabs" {
                    button type="button" class="tab" { "Account" }
                    @if is_admin {
                        button type="button" class="tab" { "Tokens" }
                    }
                }
                button type="button" id="logout" { "Log Out" }
                button type="button" id="delete" { "Delete Account" }
                button type="button" id="totp" { "Regenerate TOTP" }
            }
        }
    }
    .into()
}
