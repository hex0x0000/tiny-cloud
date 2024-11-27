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

pub fn page() -> String {
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                title { "Login Page" }
                meta name="application-name" content=(config!(server_name));
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="tcloud-prefix" content=(config!(url_prefix));
                link rel="icon" type="image/x-icon" href=(utils::make_url("/static/favicon.ico"));
                script type="text/javascript" { (webfile!("global.js")) (webfile!("login.js")) }
                style { (webfile!("global.css")) (webfile!("login.css")) }
            }
            body {
                p; div id="title" { (config!(server_name)) }
                p; div id="version" { (env!("CARGO_PKG_VERSION")) }
                p; div id="description" { (config!(description)) }
                form id="login" name="login" {
                    br; label for="user" { "Username:" }
                    br; input type="text" id="user" name="user";
                    br; label for="password" { "Password:" }
                    br; input type="password" id="password" name="password";
                    @if cfg!(feature = "totp-auth") {
                        br; label for="totp" { "TOTP Token:" }
                        br; input type="totp" id="totp" name="totp";
                    }
                    br; input value="Login" type="submit" id="btn";
                }
                div id="msg" {}
                @if config!(registration).is_some() {
                    p id="reglink" { a href=(utils::make_url("/ui/register")) { "Register Here" } }
                }
                footer {
                    br; "Tiny Cloud is licensed under the GNU Affero General Public License v3.0 or later"
                    br; a href=(env!("CARGO_PKG_REPOSITORY")) { "You can find the source code here." }
                }
            }
        }
    }
    .into()
}
