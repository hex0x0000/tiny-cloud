// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{auth::error::AuthError, plugins::error::PluginError, token::error::TokenError};
use std::convert::Into;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("IO Error: {0}")]
    IOError(String),
    #[error("Execution of SQLite command failed: {0}")]
    ExecError(String),
    #[error("User already exists")]
    UserExists,
    #[error("User was not found")]
    UserNotFound,
    #[error("Invalid username and/or id")]
    InvalidUserID,
    #[error("Time failure: {0}")]
    TimeFailure(String),
}

impl Into<AuthError> for DBError {
    fn into(self) -> AuthError {
        match self {
            Self::UserExists => AuthError::InvalidRegCredentials,
            Self::InvalidUserID | Self::UserNotFound => AuthError::InvalidSession,
            _ => AuthError::InternalError(self.to_string()),
        }
    }
}

impl Into<TokenError> for DBError {
    fn into(self) -> TokenError {
        TokenError::InternalError(self.to_string())
    }
}

impl Into<PluginError> for DBError {
    fn into(self) -> PluginError {
        PluginError::InternalError(self.to_string())
    }
}
