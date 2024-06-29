use crate::error::ErrToResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("An internal server error occurred")]
    InternalError(String),
    #[error("Token was not found")]
    NotFound,
    #[error("Token expired")]
    Expired,
}

impl ErrToResponse for TokenError {
    fn error(&self) -> &'static str {
        "TokenError"
    }

    fn err_type(&self) -> &'static str {
        match self {
            Self::InternalError(_) => stringify!(InternalError),
            Self::NotFound => stringify!(NotFound),
            Self::Expired => stringify!(Expired),
        }
    }

    fn msg(&self) -> String {
        self.to_string()
    }

    fn http_code(&self) -> u16 {
        match self {
            Self::InternalError(_) => 500,
            Self::NotFound => 404,
            Self::Expired => 401,
        }
    }

    fn handle(&self) {
        if let Self::InternalError(err) = self {
            log::error!("An internal server error occurred while handling token: {err}");
        }
    }
}
