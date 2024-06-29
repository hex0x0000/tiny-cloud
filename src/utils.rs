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
        .get(..(*config!(max_username_size) as usize))
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
