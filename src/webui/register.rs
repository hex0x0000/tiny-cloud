// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Email: hex0x0000@protonmail.com

use crate::{config, utils, web_file};
use maud::{html, Markup, PreEscaped, DOCTYPE};

#[cfg(not(feature = "totp-auth"))]
const REGISTER_JS: PreEscaped<&str> = web_file!("register.js");

#[cfg(feature = "totp-auth")]
const REGISTER_JS: PreEscaped<&str> = web_file!("register_totp.js");

#[cfg(not(feature = "totp-auth"))]
fn form() -> Markup {
    html! {
        form id="register" name="register" {
            br; label for="user" { "Username:" }
            br; input type="text" id="user" name="user";
            br; label for="password" { "Password:" }
            br; input type="password" id="password" name="password";
            br; label for="password_rep" { "Repeat Password:" }
            br; input type="password" id="password_rep" name="password_rep";
            br; label for="token" { "Registration Token:" }
            br; input type="text" id="token" name="token";
            input value="Register" type="submit" id="btn";
        }
    }
}

#[cfg(feature = "totp-auth")]
fn form() -> Markup {
    html! {
        form id="register" name="register" {
            br; label for="user" { "Username:" }
            br; input type="text" id="user" name="user";
            br; label for="password" { "Password:" }
            br; input type="password" id="password" name="password";
            br; label for="password_rep" { "Repeat Password:" }
            br; input type="password" id="password_rep" name="password_rep";
            br; label for="token" { "Registration Token:" }
            br; input type="text" id="token" name="token";
            br; label for="totp_as_qr" { "Show TOTP as a QR Code?" }
            input type="checkbox" id="totp_as_qr" name="totp_as_qr";
            br; input value="Register" type="submit" id="btn";
        }
        div id="totp" hidden {
            br; img id="totp-qr" hidden;
            div id="totp-url" {}
            p { "Save this in your TOTP app. You won't be able to access to it anymore after you click Continue." }
            button type="button" id="continue" { "Continue" }
        }
    }
}

pub fn page() -> String {
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                title { "Registration Page" }
                meta name="application-name" content=(config!(server_name));
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="tcloud-prefix" content=(config!(url_prefix));
                link rel="icon" type="image/x-icon" href=(utils::make_url("/static/favicon.ico"));
                script type="text/javascript" { (web_file!("global.js")) (REGISTER_JS) }
                style { (web_file!("global.css")) (web_file!("register.css")) }
            }
            body {
                p; div id="title" { (config!(server_name)) }
                p; div id="version" { (env!("CARGO_PKG_VERSION")) }
                p; div id="description" { (config!(description)) }
                (form())
                div id="msg" {}
            }
        }
    }
    .into()
}
