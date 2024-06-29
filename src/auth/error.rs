use crate::error::ErrToResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("An internal server error occurred")]
    InternalError(String),
    #[error("Bad credentials were given: {0}")]
    BadCredentials(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid registration credentials")]
    InvalidRegCredentials,
}

impl ErrToResponse for AuthError {
    fn error(&self) -> &'static str {
        "AuthError"
    }

    fn err_type(&self) -> &'static str {
        match self {
            Self::BadCredentials(_) => stringify!(BadCredentials),
            Self::InvalidCredentials => stringify!(InvalidCredentials),
            Self::InvalidRegCredentials => stringify!(InvalidRegCredentials),
            Self::InternalError(_) => stringify!(InternalError),
        }
    }

    fn msg(&self) -> String {
        self.to_string()
    }

    fn http_code(&self) -> u16 {
        match self {
            Self::BadCredentials(_) => 400,
            Self::InvalidCredentials => 401,
            Self::InvalidRegCredentials => 401,
            Self::InternalError(_) => 500,
        }
    }

    fn handle(&self) {
        if let Self::InternalError(err) = self {
            log::error!("An internal server error occurred during authentication: {err}");
        }
    }
}
