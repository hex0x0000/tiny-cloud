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

use rand::{rngs::StdRng, RngCore, SeedableRng};
use totp_rs::{Rfc6238, TOTP};

use crate::config;

use super::error::AuthError;

pub fn gen(user: String) -> Result<TOTP, AuthError> {
    let mut secret = [0u8; 16];
    StdRng::from_entropy().fill_bytes(&mut secret);
    let mut rfc =
        Rfc6238::with_defaults(secret.to_vec()).map_err(|e| AuthError::InternalError(format!("Failed to generate new TOTP: {e}")))?;
    rfc.issuer(config!(server_name).replace(":", ""));
    rfc.account_name(user);
    TOTP::from_rfc6238(rfc).map_err(|e| AuthError::InternalError(format!("Failed to generate new TOTP: {e}")))
}

pub fn check(totp: String, token: String) -> Result<(), AuthError> {
    let totp = TOTP::from_url(totp).map_err(|e| AuthError::InternalError(format!("Invalid TOTP url was given: {e}")))?;
    if totp
        .check_current(&token)
        .map_err(|e| AuthError::InternalError(format!("Time error: {e}")))?
    {
        Ok(())
    } else {
        Err(AuthError::InvalidTOTP)
    }
}
