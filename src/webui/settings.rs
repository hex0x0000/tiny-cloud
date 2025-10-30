// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{config, utils, webfile, webui::home::header};
use maud::{DOCTYPE, PreEscaped, html};

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
                (header(is_admin))
                div id="settings" {
                    p id="msg" {
                        "Hi " (username) "! This is your settings page." br;
                    }
                    button type="button" class="setting" id="logout" { "Log Out" }
                    button type="button" class="setting" { "Recreate TOTP" }
                    form id="totp" name="totp" {
                        h3 { "Here you can change your TOTP secret" }
                        br; label for="tpasswd" { "Insert password:" }
                        br; input type="password" id="tpasswd" name="tpasswd" required;
                        br; input type="checkbox" id="totp_as_qr" name="totp_as_qr" checked;
                        label for="totp_as_qr" { "Receive TOTP secret as a QR Code?" }
                        br; input value="Change TOTP" type="submit";
                    }
                    div id="totp-res" hidden {
                        br; img id="totp-qr" src="";
                        br; p id="totp-url" { "" }
                        br; button id="totp-btn" { "I saved the TOTP secret" }
                    }
                    button type="button" class="setting" id="passwd" { "Change Password" }
                    form id="changepwd" name="changepwd" {
                        h3 { "If you need to change password but you don't remember it you can ask the admin to create a token for changing your password. Check 'This is a token' if you are using a token" }
                        br; label for="oldpassword" { "Insert old password or token:" }
                        br; input type="password" id="oldpassword" name="oldpassword" required;
                        input type="checkbox" id="istoken" name="istoken";
                        label for="istoken" { "This is a token" }
                        br; label for="new_password" { "Insert new password:" }
                        br; input type="password" id="new_password" name="new_password" required;
                        br; label for="newpasswd_rep" { "Confirm new password:" }
                        br; input type="password" id="newpasswd_rep" name="newpasswd_rep" required;
                        br; input value="Change password" type="submit";
                    }
                    button type="button" class="setting" id="session" { "Log out all Sessions" }
                    button type="button" class="setting" id="delete" { "Delete Account" }
                }
            }
        }
    }
    .into()
}
