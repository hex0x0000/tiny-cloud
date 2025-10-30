// SPDX-License-Identifier: AGPL-3.0-or-later

use super::error::AuthError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, errors, rand_core::OsRng},
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
pub async fn verify(password: &[u8], hash: String) -> Result<(), AuthError> {
    let password = Zeroizing::new(password.to_vec());
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
pub async fn create(password: &[u8]) -> Result<String, AuthError> {
    let password = Zeroizing::new(password.to_vec());
    task::spawn_blocking(move || create_blocking(&password))
        .await
        .map_err(|e| AuthError::InternalError(format!("Hash creation task failed: {e}")))?
}
