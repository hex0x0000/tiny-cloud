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

pub mod cli;
pub mod error;
mod hash;
mod totp;

use crate::api::auth::Login;
use crate::config;
use crate::database::auth;
use crate::token;
use actix_identity::Identity;
use actix_identity::error::GetIdentityError;
use async_sqlite::Pool;
use common_library::error::ErrToResponse;
use error::AuthError;
use totp_rs::TOTP;

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
            return Err(AuthError::BadCredentials("Username must be alphanumeric".into()));
        }
    }
    Ok(())
}

/// Checks a user's password and validates the TOTP token.
/// Returns user's username on success.
pub async fn check(pool: &Pool, login: Login) -> Result<String, AuthError> {
    check_validity(&login.user, login.password.as_bytes())?;
    let dummy_hash = hash::create(login.password.as_bytes()).await?;
    match auth::get_auth(pool, login.user.clone()).await.map_err(|e| e.into())? {
        Some(user) => {
            hash::verify(login.password.as_bytes(), user.pass_hash).await?;
            self::totp::check(user.totp, &login.totp)?;
            Ok(user.userid)
        }
        None => {
            // Dummy verification to keep the same response timings when the user is not found.
            // Keeps malicious attackers from scanning the server for usernames
            let _ = hash::verify(login.password.as_bytes(), dummy_hash).await;
            Err(AuthError::InvalidCredentials)
        }
    }
}

/// Adds a new user and returns its TOTP. Fails if username already exists.
/// Used when adding user manually.
pub async fn add_user(pool: &Pool, username: String, password: &[u8], is_admin: bool) -> Result<TOTP, AuthError> {
    check_validity(&username, password)?;
    let passwd_hash = hash::create(password).await?;
    let totp = self::totp::generate(username.clone())?;
    auth::add_user(pool, username, passwd_hash, totp.get_url(), is_admin)
        .await
        .map_err(|e| e.into())?;
    Ok(totp)
}

/// Registers a new user with a token and returns its TOTP, and returns the userid used during the
/// session.
/// Fails if username already exists or if token is not valid
pub async fn register_user(
    pool: &Pool,
    username: String,
    password: &[u8],
    token: String,
) -> Result<(TOTP, String), Box<dyn ErrToResponse>> {
    check_validity(&username, password).map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    token::check_token(pool, token)
        .await
        .map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    let passwd_hash = hash::create(password).await.map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    let totp = self::totp::generate(username.clone()).map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    let userid = auth::add_user(pool, username, passwd_hash, totp.get_url(), false)
        .await
        .map_err(|e| Box::new(Into::<AuthError>::into(e)) as Box<dyn ErrToResponse>)?;
    Ok((totp, userid))
}

/// Unwraps id and returns its string or its error as a response
pub fn id_err_into(err: GetIdentityError) -> AuthError {
    match err {
        GetIdentityError::SessionGetError(err) => {
            AuthError::InternalError(format!("Failed to accessing the session store while validating identity: {err}"))
        }
        GetIdentityError::LostIdentityError(err) => {
            AuthError::InternalError(format!("Identity info was lost after being validated (Actix Identity bug): {err}"))
        }
        _ => AuthError::InvalidSession,
    }
}

/// Deletes user and logs out
pub async fn delete_user(pool: &Pool, user: Identity) -> Result<(), AuthError> {
    auth::delete_user(pool, user.id().map_err(|e| id_err_into(e))?)
        .await
        .inspect(|_| user.logout())
        .map_err(|e| e.into())
}

/// If the user is some, checks whether the userid is valid or not and returns
/// Checks whether the userid is valid or not and returns the user's username and admin status,
/// and if it is not valid it terminates the session.
pub async fn validate_user(pool: &Pool, user: Identity) -> Result<(String, bool), AuthError> {
    auth::userinfo(pool, user.id().map_err(|e| id_err_into(e))?)
        .await
        .map_err(|e| e.into())
        .and_then(|userinfo| userinfo.ok_or(AuthError::InvalidSession))
        .inspect_err(|_| user.logout())
}

/// Changes user's sessionid. Logs out on success.
pub async fn change_sessionid(pool: &Pool, user: Identity) -> Result<(), AuthError> {
    auth::change_sessionid(pool, user.id().map_err(|e| id_err_into(e))?)
        .await
        .map_err(|e| e.into())
        .inspect(|_| user.logout())
}

/// Changes user's password.
/// After the password has been changed it changes sessionid to log out older sessions.
/// If the session is not correct it logs out.
pub async fn change_pwd(pool: &Pool, user: Identity, new_pwd: &[u8], old_pwd: &[u8]) -> Result<(), AuthError> {
    let (username, sessionid) = auth::unpack(user.id().map_err(|e| id_err_into(e))?).map_err(|e| e.into())?;
    match auth::get_passhash(pool, username.clone(), sessionid).await.map_err(|e| e.into())? {
        Some(pass_hash) => {
            hash::verify(old_pwd, pass_hash).await?;
            let new_pwd = hash::create(new_pwd).await?;
            auth::change_passhash(pool, username, new_pwd).await.map_err(|e| e.into())?;
            change_sessionid(pool, user).await
        }
        None => {
            user.logout();
            Err(AuthError::InvalidSession)
        }
    }
}

/// Changes user's password using a token.
pub async fn change_pwd_token(pool: &Pool, user: Identity, new_pwd: &[u8], token: String) -> Result<(), Box<dyn ErrToResponse>> {
    let (username, _) = validate_user(pool, user).await.map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;

    token::check_pwd_token(pool, token, username.clone())
        .await
        .map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;

    let new_pwd = hash::create(new_pwd).await.map_err(|e| Box::new(e) as Box<dyn ErrToResponse>)?;
    auth::change_passhash(pool, username, new_pwd)
        .await
        .map_err(|e| Box::new(Into::<AuthError>::into(e)) as Box<dyn ErrToResponse>)?;

    Ok(())
}

/// Regenerates TOTP secret, logs out from all sessions and returns the new secret to be sent to the client.
pub async fn change_totp(pool: &Pool, user: Identity, pwd: &[u8]) -> Result<TOTP, AuthError> {
    let (username, sessionid) = auth::unpack(user.id().map_err(|e| id_err_into(e))?).map_err(|e| e.into())?;
    match auth::get_passhash(pool, username.clone(), sessionid).await.map_err(|e| e.into())? {
        Some(pass_hash) => {
            hash::verify(pwd, pass_hash).await?;
            let new_totp = totp::generate(username.clone())?;
            auth::change_totp(pool, username, new_totp.get_url()).await.map_err(|e| e.into())?;
            change_sessionid(pool, user).await?;
            Ok(new_totp)
        }
        None => {
            user.logout();
            Err(AuthError::InvalidSession)
        }
    }
}
