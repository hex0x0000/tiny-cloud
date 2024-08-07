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

use super::error::AuthError;
use argon2::{
    password_hash::{errors, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tokio::task;
use zeroize::Zeroizing;

fn verify_blocking(password: &[u8], hash: &str) -> Result<(), AuthError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| AuthError::InternalError(format!("Failed to parse password hash: {e}")))?;
    match Argon2::default().verify_password(password, &parsed_hash) {
        Ok(_) => Ok(()),
        Err(err) => match err {
            errors::Error::Password => Err(AuthError::InvalidCredentials),
            _ => Err(AuthError::InternalError(format!("Failed to verify password: {err}"))),
        },
    }
}

/// Verifies password's correctness.
/// Runs Argon2 on a blocking task to avoid starving the async pool
pub async fn verify(password: Zeroizing<Vec<u8>>, hash: String) -> Result<(), AuthError> {
    task::spawn_blocking(move || verify_blocking(&password, &hash))
        .await
        .map_err(|e| AuthError::InternalError(format!("Hash verification task failed: {e}")))?
}

fn create_blocking(password: &[u8]) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password, &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AuthError::InternalError(format!("Failed to hash password: {e}")))
}

/// Creates a new hash from the user's password.
/// Runs Argon2 on a blocking task to avoid starving the async pool
pub async fn create(password: Zeroizing<Vec<u8>>) -> Result<String, AuthError> {
    task::spawn_blocking(move || create_blocking(&password))
        .await
        .map_err(|e| AuthError::InternalError(format!("Hash creation task failed: {e}")))?
}
