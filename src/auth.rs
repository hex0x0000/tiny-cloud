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

pub mod cli;
pub mod error;
mod hash;
#[cfg(feature = "totp-auth")]
mod totp;

use crate::api::auth::Login;
use crate::config;
use crate::database;
use crate::token::check_token;
use async_sqlite::Pool;
use database::auth;
use error::AuthError;
use tcloud_library::error::ErrToResponse;
#[cfg(feature = "totp-auth")]
use totp_rs::TOTP;
use zeroize::Zeroizing;

fn check_validity(username: &str, password: &[u8]) -> Result<(), AuthError> {
    let user_len = username.len();
    let passwd_len = password.len();
    let max_username_size = *config!(cred_size.max_username) as usize;
    let min_username_size = *config!(cred_size.min_username) as usize;
    let max_passwd_size = *config!(cred_size.max_passwd) as usize;
    let min_passwd_size = *config!(cred_size.min_passwd) as usize;
    if user_len > max_username_size || user_len < min_username_size {
        return Err(AuthError::BadCredentials(format!(
            "Accepted username size is between {min_username_size} and {max_username_size} characters",
        )));
    }
    if passwd_len > max_passwd_size || passwd_len < min_passwd_size {
        return Err(AuthError::BadCredentials(format!(
            "Accepted password length is between {min_passwd_size} and {max_passwd_size} bytes",
        )));
    }
    for c in username.chars() {
        if !c.is_alphanumeric() {
            return Err(AuthError::BadCredentials("Username must be alphanumerical".into()));
        }
    }
    Ok(())
}

/// Checks a user's password
#[cfg(not(feature = "totp-auth"))]
pub async fn check(pool: &Pool, login: Login) -> Result<String, AuthError> {
    let password = Zeroizing::new(login.password.into_bytes());
    check_validity(&login.user, &password)?;
    let dummy_hash = hash::create(password.clone()).await?;
    match auth::get_auth(pool, login.user.clone()).await.map_err(|e| e.into())? {
        Some(user) => {
            hash::verify(password, user.pass_hash).await?;
            Ok(login.user)
        }
        None => {
            // Dummy verification to keep the same response timings when the user is not found.
            // Keeps malicious attackers from scanning the server for usernames
            let _ = hash::verify(password, dummy_hash).await;
            Err(AuthError::InvalidCredentials)
        }
    }
}

/// Checks a user's password and validates the TOTP token.
/// Returns user's username on success.
#[cfg(feature = "totp-auth")]
pub async fn check(pool: &Pool, login: Login) -> Result<String, AuthError> {
    let password = Zeroizing::new(login.password.into_bytes());
    check_validity(&login.user, &password)?;
    let dummy_hash = hash::create(password.clone()).await?;
    match auth::get_auth(pool, login.user.clone()).await.map_err(|e| e.into())? {
        Some(user) => {
            hash::verify(password, user.pass_hash).await?;
            self::totp::check(user.totp, login.totp)?;
            Ok(login.user)
        }
        None => {
            // Dummy verification to keep the same response timings when the user is not found.
            // Keeps malicious attackers from scanning the server for usernames
            let _ = hash::verify(password, dummy_hash).await;
            Err(AuthError::InvalidCredentials)
        }
    }
}

/// Adds a new user. Fails if username already exists
#[cfg(not(feature = "totp-auth"))]
pub async fn add_user(pool: &Pool, username: String, password: Zeroizing<Vec<u8>>, is_admin: bool) -> Result<(), AuthError> {
    check_validity(&username, &password)?;
    let passwd_hash = hash::create(password).await?;
    auth::add_user(pool, username, passwd_hash, is_admin).await.map_err(|e| e.into())?;
    Ok(())
}

/// Registers a new user with a token.
/// Fails if username already exists or if token is not valid
#[cfg(not(feature = "totp-auth"))]
pub async fn register_user(
    pool: &Pool,
    username: String,
    password: Zeroizing<Vec<u8>>,
    token: String,
) -> Result<(), Box<dyn ErrToResponse>> {
    check_validity(&username, &password).map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    check_token(pool, token).await.map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    let passwd_hash = hash::create(password).await.map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    auth::add_user(pool, username, passwd_hash, false)
        .await
        .map_err(|e| Box::new(Into::<AuthError>::into(e)) as Box<dyn ErrToResponse>)?;
    Ok(())
}

/// Adds a new user and returns its TOTP. Fails if username already exists
#[cfg(feature = "totp-auth")]
pub async fn add_user(pool: &Pool, username: String, password: Zeroizing<Vec<u8>>, is_admin: bool) -> Result<TOTP, AuthError> {
    check_validity(&username, &password)?;
    let passwd_hash = hash::create(password).await?;
    let totp = self::totp::gen(username.clone())?;
    auth::add_user(pool, username, passwd_hash, totp.get_url(), is_admin)
        .await
        .map_err(|e| e.into())?;
    Ok(totp)
}

/// Registers a new user with a token and returns its TOTP.
/// Fails if username already exists or if token is not valid
#[cfg(feature = "totp-auth")]
pub async fn register_user(
    pool: &Pool,
    username: String,
    password: Zeroizing<Vec<u8>>,
    token: String,
) -> Result<TOTP, Box<dyn ErrToResponse>> {
    check_validity(&username, &password).map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    check_token(pool, token).await.map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    let passwd_hash = hash::create(password).await.map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    let totp = self::totp::gen(username.clone()).map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    auth::add_user(pool, username, passwd_hash, totp.get_url(), false)
        .await
        .map_err(|e| Box::new(Into::<AuthError>::into(e)) as Box<dyn ErrToResponse>)?;
    Ok(totp)
}

pub async fn delete_user(pool: &Pool, username: String) -> Result<(), AuthError> {
    auth::delete_user(&pool, username).await.map_err(|e| e.into())?;
    Ok(())
}
