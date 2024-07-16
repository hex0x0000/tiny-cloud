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
            br; label for="token" { "Token:" }
            br; input type="text" id="token" name="token";
            input value="Register" type="submit";
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
            br; label for="token" { "Token:" }
            br; input type="text" id="token" name="token";
            br; label for="totp_as_qr" { "Show TOTP as a qr?" }
            br; input type="checkbox" id="totp_as_qr" name="totp_as_qr";
            input value="Register" type="submit";
        }
        div id="totp" hidden {
            br;
            img id="totp-qr" hidden;
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
                div id="msg";
            }
        }
    }
    .into()
}
