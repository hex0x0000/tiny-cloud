use crate::{config, utils, web_file};
use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn page() -> Markup {
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
                script type="text/javascript" { (web_file!("register.js")) }
                style { (web_file!("global.css")) (web_file!("register.css")) }
            }
            body {
                div { "Test Test" }
            }
        }
    }
}
