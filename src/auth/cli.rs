// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::auth::{add_user, error::AuthError};
use async_sqlite::Pool;
use std::io::{self, Write};
use zeroize::{Zeroize, Zeroizing};

pub async fn create_user(pool: &Pool) -> Result<(), String> {
    use std::{fs::File, path::PathBuf};

    let mut input = String::new();

    // Gets user from CLI
    print!("User: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read user");
    let user = input.trim().to_string();

    // Gets password from CLI using a safe input
    let password = {
        let first = rpassword::prompt_password("Password: ")
            .expect("Failed to read password")
            .into_bytes();
        let mut second = rpassword::prompt_password("Confirm password: ")
            .expect("Confirm password")
            .into_bytes();
        if first != second {
            second.zeroize();
            return Err("Passwords do not match.".into());
        } else {
            second.zeroize();
            Zeroizing::new(first)
        }
    };
    let pass_len = password.len();

    // Make user admin?
    print!("Make user admin? [y/n] ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read admin request");
    let is_admin = input.trim().to_string().to_lowercase() == "y";

    // Add user to DB
    let totp = add_user(&pool, user.clone(), &password, is_admin).await.map_err(|e| match e {
        AuthError::InvalidRegCredentials => format!("{e}"),
        AuthError::InternalError(ref err) => format!("{e}: {err}"),
        _ => e.to_string(),
    })?;

    if is_admin {
        println!("Successfully added admin {} with password length {}", user, pass_len);
    } else {
        println!("Successfully added user {} with password length {}", user, pass_len);
    }

    print!("Insert path to output the TOTP's QR code image (png), if you want to get it as a URL leave empty: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read TOTP request");
    let path = input.trim().to_string();
    if path.is_empty() {
        println!("{}", totp.get_url());
    } else {
        let mut path = PathBuf::from(path);
        path.push(format!("{user}-totp-qr.png"));
        let mut qr_file = File::options()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|e| format!("Failed to open file for the QR code image: {e}"))?;
        qr_file
            .write_all(&totp.get_qr_png()?)
            .map_err(|e| format!("Failed to write QR code image: {e}"))?;
        println!("QR code image written.");
    }

    Ok(())
}
