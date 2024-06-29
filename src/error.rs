use actix_web::{http::StatusCode, HttpResponse};
use serde_json::json;

/// Common trait for errors that must be sent to the client through an http response
pub trait ErrToResponse {
    /// Error's name
    fn error(&self) -> &'static str;

    /// Error's type
    fn err_type(&self) -> &'static str;

    /// Error's message
    fn msg(&self) -> String;

    /// Error's http code
    fn http_code(&self) -> u16;

    /// Handles special types before building the response
    fn handle(&self);

    /// Turns the error into a response
    fn to_response(&self) -> HttpResponse {
        self.handle();
        HttpResponse::build(
            StatusCode::from_u16(self.http_code())
                .expect("Invalid http code returned by http_code(). This is a bug"),
        )
        .content_type("application/json")
        .body(
            json!({
                "error": self.error(),
                "type": self.err_type(),
                "msg": self.msg()
            })
            .to_string(),
        )
    }
}
