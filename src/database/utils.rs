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
