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

use log::LevelFilter;

mutually_exclusive_features::exactly_one_of!("default-log", "syslog", "systemd-log");

pub fn get_filter(level: &str) -> Result<LevelFilter, String> {
    match level {
        "off" => Ok(LevelFilter::Off),
        "trace" => Ok(LevelFilter::Trace),
        "debug" => Ok(LevelFilter::Debug),
        "info" => Ok(LevelFilter::Info),
        "warn" => Ok(LevelFilter::Warn),
        "error" => Ok(LevelFilter::Error),
        _ => Err(format!(
            "'{level}' is not a valid filter. Accepted values are: `off`, `trace`, `debug`, `info`, `warn`, `error`."
        )),
    }
}

#[cfg(feature = "syslog")]
pub fn init() -> Result<(), String> {
    use crate::config;
    use syslog::{BasicLogger, Formatter3164};

    let logger = syslog::unix(Formatter3164::default()).map_err(|e| format!("Failed to connect to syslog: {e}"))?;
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(get_filter(config!(logging.log_level))?))
        .map_err(|e| format!("Failed to set up syslog logger: {e}"))?;
    Ok(())
}

#[cfg(feature = "systemd-log")]
pub fn init() -> Result<(), String> {
    use crate::config;
    use systemd_journal_logger::JournalLog;

    JournalLog::new()
        .map_err(|e| format!("Failed to instatiate systemd journal log: {e}"))?
        .install()
        .map_err(|e| format!("Failed to install systemd journal log: {e}"))?;
    log::set_max_level(get_filter(config!(logging.log_level))?);
    Ok(())
}
