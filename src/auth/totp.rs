use rand::{rngs::StdRng, RngCore, SeedableRng};
use totp_rs::{Rfc6238, TOTP};

use crate::config;

use super::error::AuthError;

pub fn gen(user: String) -> Result<TOTP, AuthError> {
    let mut secret = [0u8; 16];
    StdRng::from_entropy().fill_bytes(&mut secret);
    let mut rfc = Rfc6238::with_defaults(secret.to_vec())
        .map_err(|e| AuthError::InternalError(format!("Failed to generate new TOTP: {e}")))?;
    rfc.issuer(config!(server_name).replace(":", ""));
    rfc.account_name(user);
    TOTP::from_rfc6238(rfc)
        .map_err(|e| AuthError::InternalError(format!("Failed to generate new TOTP: {e}")))
}

pub fn check(totp: String, token: String) -> Result<(), AuthError> {
    let totp = TOTP::from_url(totp)
        .map_err(|e| AuthError::InternalError(format!("Invalid TOTP url was given: {e}")))?;
    if totp
        .check_current(&token)
        .map_err(|e| AuthError::InternalError(format!("Time error: {e}")))?
    {
        Ok(())
    } else {
        Err(AuthError::InvalidTOTP)
    }
}
