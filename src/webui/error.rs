// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{config, utils, webfile};
use actix_web::HttpResponse;
use common_library::error::ErrToResponse;
use maud::{DOCTYPE, PreEscaped, html};

fn to_page<E: ErrToResponse>(err: E, code: u16) -> String {
    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                title { "Home Page" }
                meta name="application-name" content=(config!(server_name));
                meta charset="UTF-8";
                meta name="tcloud-prefix" content=(config!(url_prefix));
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" type="image/x-icon" href=(utils::make_url("/static/favicon.ico"));
                style {
                    (webfile!("global.css"))
                    (webfile!("error.css"))
                }
            }
            body {
                h1 { "Uh Oh :(" }
                p id="name" { "An error occurred: " strong { (err.error()) } }
                p id="msg" { (err.msg()) }
                img src=(format!("https://http.cat/{code}")) alt=(format!("HTTP Code {code}")) width="530" height="424";
            }
        }
    }
    .into()
}

pub fn to_response_page<E: ErrToResponse>(err: E) -> HttpResponse {
    let code = err.http_code().finish().status().as_u16();
    err.http_code().body(to_page(err, code))
}
