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
