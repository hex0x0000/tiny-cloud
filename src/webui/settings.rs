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

use crate::{
    config, utils, webfile,
    webui::home::{NAVBAR_ADMIN, NAVBAR_USER},
};
use maud::{html, PreEscaped, DOCTYPE};

pub fn page(username: String, is_admin: bool) -> String {
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                title { "Settings" }
                meta name="application-name" content=(config!(server_name));
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="tcloud-prefix" content=(config!(url_prefix));
                link rel="icon" type="image/x-icon" href=(utils::make_url("/static/favicon.ico"));
                script type="text/javascript" {
                    (webfile!("global.js"))
                    (webfile!("navbar.js"))
                    (webfile!("settings.js"))
                }
                style { (webfile!("global.css")) (webfile!("navbar.css")) (webfile!("settings.css")) }
            }
            body {
                @if is_admin {
                    (*NAVBAR_ADMIN)
                } @else {
                    (*NAVBAR_USER)
                }
                div id="settings" {
                    p id="msg" {
                        "Hi " (username) "! This is your settings page." br;
                        "If you need to change password but you don't remember it you can ask the admin to create a token for changing your password"
                    }
                    button type="button" class="setting" id="logout" { "Log Out" }
                    button type="button" class="setting" id="totp" { "Recreate TOTP" }
                    form id="totp" name="totp" {
                        br; label for="tpasswd" { "Insert password:" }
                        br; input type="text" id="tpasswd" name="tpasswd";
                        br; input value="Change TOTP" type="submit" id="tbtn";
                    }
                    button type="button" class="setting" id="passwd" { "Change Password" }
                    form id="passwd" name="passwd" {
                        br; label for="oldpasswd" { "Insert old password or token:" }
                        br; input type="text" id="oldpasswd" name="oldpasswd";
                        br; label for="newpasswd" { "Insert new password:" }
                        br; input type="text" id="newpasswd" name="newpasswd";
                        br; label for="newpasswd_rep" { "Confirm new password:" }
                        br; input type="text" id="newpasswd_rep" name="newpasswd_rep";
                        br; input value="Change password" type="submit" id="tbtn";
                    }
                    button type="button" class="setting" id="session" { "Invalidate Session" }
                    button type="button" class="setting" id="delete" { "Delete Account" }
                }
            }
        }
    }
    .into()
}
