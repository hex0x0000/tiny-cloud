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
use actix_web::HttpResponse;
use maud::{html, PreEscaped, DOCTYPE};
use tcloud_library::error::ErrToResponse;

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
