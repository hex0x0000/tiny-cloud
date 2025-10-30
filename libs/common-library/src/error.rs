// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{HttpResponse, HttpResponseBuilder};
use serde_json::json;

/// Common trait for errors that can be returned to the client.
pub trait ErrToResponse {
    /// Error's name
    fn error(&self) -> &'static str;

    /// Error's type
    fn err_type(&self) -> &'static str;

    /// Error's message
    fn msg(&self) -> String;

    /// Error's http code
    fn http_code(&self) -> HttpResponseBuilder;

    /// Handles special types before building the response
    fn handle(&self);

    /// Turns the error into a response
    fn to_response(&self) -> HttpResponse {
        self.handle();
        self.http_code().content_type("application/json").body(
            json!({
                "error": self.error(),
                "type": self.err_type(),
                "msg": self.msg()
            })
            .to_string(),
        )
    }
}
