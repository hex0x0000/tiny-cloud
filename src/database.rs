pub mod auth;
pub mod error;
pub mod token;
pub mod utils;
use crate::config;
use async_sqlite::{JournalMode, Pool, PoolBuilder};
use auth::USERS_TABLE;
use error::DBError;
use std::path::PathBuf;
use token::TOKEN_TABLE;
use tokio::fs;

fn get_tables() -> String {
    if config!(registration).is_some() {
        format!("BEGIN;\n{USERS_TABLE}\n{TOKEN_TABLE}\nCOMMIT;")
    } else {
        format!("BEGIN;\n{USERS_TABLE}\nCOMMIT;")
    }
}

/// Connects to sqlite database and returns a pool.
/// Sets it to Wal mode by default, which is better for concurrency.
pub async fn init_db() -> Result<Pool, DBError> {
    fs::create_dir_all(config!(data_directory))
        .await
        .map_err(|e| DBError::IOError(format!("Failed to create data data directory: {e}")))?;
    let mut data_path = PathBuf::from(config!(data_directory));
    data_path.push("auth.db");
    let pool = PoolBuilder::new()
        .journal_mode(JournalMode::Wal)
        .path(data_path)
        .open()
        .await
        .map_err(|e| DBError::IOError(e.to_string()))?;
    pool.conn(|conn| conn.execute_batch(&get_tables()))
        .await
        .map_err(|e| DBError::ExecError(format!("Failed to initialize tables: {e}")))?;
    Ok(pool)
}
