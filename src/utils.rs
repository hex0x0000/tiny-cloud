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

use crate::config;
use actix_identity::error::GetIdentityError;
use actix_web::{dev::ConnectionInfo, HttpResponse};

/// Creates URL using the prefix specified in settings
pub fn make_url(url: &str) -> String {
    let prefix = config!(url_prefix);
    if prefix.is_empty() {
        url.into()
    } else {
        format!("/{}{}", prefix, url)
    }
}

/// Gets ip of connection's info from the most reliable source
/// depending on wether or not the server is behind a proxy
pub fn get_ip(conn: &ConnectionInfo) -> &str {
    if *config!(server.is_behind_proxy) {
        conn.realip_remote_addr()
    } else {
        conn.peer_addr()
    }
    .unwrap_or("unknown")
}

/// Sanitizes a username to make it safe to log or display
pub fn sanitize_user(username: &str) -> String {
    username
        .get(..(*config!(cred_size.max_username) as usize))
        .unwrap_or(username)
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

/// Unwraps id and returns its string or its error as a response
pub fn id_err_into(err: GetIdentityError) -> HttpResponse {
    match err {
        GetIdentityError::SessionGetError(err) => {
            log::error!("Failed to accessing the session store while validating identity: {err}");
            HttpResponse::InternalServerError().body("")
        }
        GetIdentityError::LostIdentityError(err) => {
            log::error!("Identity info was lost after being validated (Actix Identity bug): {err}");
            HttpResponse::InternalServerError().body("")
        }
        _ => HttpResponse::Forbidden().body(""),
    }
}
