use crate::{auth::error::AuthError, token::error::TokenError};
use std::convert::Into;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("IO Error: `{0}`")]
    IOError(String),
    #[error("Execution of SQLite command failed: `{0}`")]
    ExecError(String),
    #[error("User already exists")]
    UserExists,
    #[error("Feature `{0}` is not enabled")]
    NotEnabled(String),
    #[error("Time failure: {0}")]
    TimeFailure(String),
}

impl Into<AuthError> for DBError {
    fn into(self) -> AuthError {
        match self {
            Self::UserExists => AuthError::InvalidRegCredentials,
            _ => AuthError::InternalError(self.to_string()),
        }
    }
}

impl Into<TokenError> for DBError {
    fn into(self) -> TokenError {
        TokenError::InternalError(self.to_string())
    }
}
