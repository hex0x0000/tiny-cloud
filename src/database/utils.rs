// SPDX-License-Identifier: AGPL-3.0-or-later

use super::error::DBError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn now() -> Result<u64, DBError> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| DBError::TimeFailure("System clock may have gone backwards".into()))?
        .as_secs())
}

pub fn calc_expire(duration: Duration) -> Result<u64, DBError> {
    Ok(SystemTime::now()
        .checked_add(duration)
        .ok_or(DBError::TimeFailure("Failed to calculate expire date".into()))?
        .duration_since(UNIX_EPOCH)
        .map_err(|_| DBError::TimeFailure("System clock may have gone backwards".into()))?
        .as_secs())
}
