use crate::{config, utils, web_file};
use maud::{html, PreEscaped, DOCTYPE};

pub fn page(username: String) -> String {
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                title { "Main Page" }
                meta name="application-name" content=(config!(server_name));
                meta charset="utf-8";
                meta name="tcloud-prefix" content=(config!(url_prefix));
                meta name="tcloud-username" content=(username);
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" type="image/x-icon" href=(utils::make_url("/static/favicon.ico"));
            }
            body {
                h1 { "Hi " (username) }
            }
        }
    }
    .into()
}
