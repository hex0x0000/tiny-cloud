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

use crate::config;
use log::LevelFilter;
#[cfg(feature = "normal-log")]
use simplelog::*;
#[cfg(feature = "normal-log")]
use std::fs::OpenOptions;
#[cfg(feature = "syslog")]
use syslog::{BasicLogger, Formatter3164};
#[cfg(feature = "systemd-log")]
use systemd_journal_logger::JournalLog;

fn get_filter(level: &str) -> LevelFilter {
    match level {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => {
            eprintln!("Disabling level filtering");
            LevelFilter::Off
        }
    }
}

mutually_exclusive_features::exactly_one_of!("normal-log", "syslog", "systemd-log");

#[cfg(feature = "normal-log")]
pub fn init_logging() -> Result<(), String> {
    let level_filter = get_filter(config!(logging.log_level));
    if let Some(file) = config!(logging.file) {
        CombinedLogger::init(vec![
            if *config!(logging.terminal) {
                TermLogger::new(level_filter, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
            } else {
                SimpleLogger::new(level_filter, Config::default())
            },
            WriteLogger::new(
                level_filter,
                Config::default(),
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(file)
                    .map_err(|e| format!("Failed to open log file `{file}`: {e}"))?,
            ),
        ])
        .map_err(|e| format!("Failed to initialize combined logger: {e}"))?;
    } else if *config!(logging.terminal) {
        TermLogger::init(level_filter, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
            .map_err(|e| format!("Failed to initialize terminal logger: {e}"))?;
    } else {
        SimpleLogger::init(level_filter, Config::default()).map_err(|e| format!("Failed to initialize simple logger: {e}"))?;
    }
    Ok(())
}

#[cfg(feature = "syslog")]
pub fn init_logging() -> Result<(), String> {
    let logger = syslog::unix(Formatter3164::default()).map_err(|e| format!("Failed to connect to syslog: {e}"))?;
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(get_filter(config!(logging.log_level))))
        .map_err(|e| format!("Failed to set up syslog logger: {e}"))?;
    Ok(())
}

#[cfg(feature = "systemd-log")]
pub fn init_logging() -> Result<(), String> {
    JournalLog::new()
        .map_err(|e| format!("Failed to create journal log: {e}"))?
        .install()
        .map_err(|e| format!("Failed to install journal log: {e}"))?;
    log::set_max_level(get_filter(config!(logging.log_level)));
    Ok(())
}
