use crate::{config, utils, web_file};
use maud::{html, Markup, PreEscaped, DOCTYPE};

fn registration_link() -> Markup {
    if config!(registration).is_some() {
        html! {
            a href=(utils::make_url("/ui/register")) { "Register Here" }
        }
    } else {
        html!()
    }
}

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
                script type="text/javascript" { (web_file!("global.js")) (web_file!("login.js")) }
                style { (web_file!("global.css")) (web_file!("login.css")) }
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
                    input value="Login" type="submit";
                }
                div id="msg" {}
                div id="reglink" { (registration_link()) }
            }
        }
    }
    .into()
}
