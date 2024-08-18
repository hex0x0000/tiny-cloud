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

mod api;
mod auth;
mod config;
mod database;
mod error;
mod plugins;
mod server;
#[cfg(not(feature = "no-tls"))]
mod tls;
mod token;
mod utils;
mod webui;
#[macro_use]
mod macros;
use std::process::ExitCode;

use actix_web::cookie::Key;
use log::LevelFilter;
use plugins::Plugins;
use tcloud_library::tiny_args::*;
use tokio::fs;
use zeroize::Zeroizing;

#[actix_web::main]
async fn main() -> ExitCode {
    let mut exit = ExitCode::SUCCESS;
    if let Err(e) = run().await {
        eprintln!("{e}");
        log::error!("{e}");
        exit = ExitCode::FAILURE
    }
    tiny_logs::end().await;
    exit
}

pub fn log_filter(level: &str) -> Result<LevelFilter, String> {
    match level {
        "off" => Ok(LevelFilter::Off),
        #[cfg(debug_assertions)]
        "trace" => Ok(LevelFilter::Trace),
        #[cfg(debug_assertions)]
        "debug" => Ok(LevelFilter::Debug),
        "info" => Ok(LevelFilter::Info),
        "warn" => Ok(LevelFilter::Warn),
        "error" => Ok(LevelFilter::Error),
        #[cfg(not(debug_assertions))]
        "trace" | "debug" => {
            Err("`trace` and `debug` logs are disabled on release. Compile without the `--release` flag to enable them.".into())
        }
        _ => Err(format!(
            "'{level}' is not a valid log filter. Accepted values are: `off`, `trace`, `debug`, `info`, `warn`, `error`."
        )),
    }
}

async fn run() -> Result<(), String> {
    let mut plugins = Plugins::new();

    let mut cmd = Command::create("tiny-cloud", env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .license(env!("CARGO_PKG_LICENSE"))
        .arg(
            arg! { -c, --config },
            ArgType::String,
            "Path to the configuration file (default: ./config.toml)",
        )
        .arg(arg! { --create-user }, ArgType::Flag, "Creates a new user and exits")
        .arg(
            arg! { --write-default },
            ArgType::Flag,
            "Writes the default configuration and exits",
        )
        .arg(arg! { -h, --help }, ArgType::Flag, "Shows this help and exits");
    cmd = plugins.add_subcmds(cmd);
    let parsed = cmd.build().parse()?;

    if plugins.handle_args(&parsed) {
        return Ok(());
    }

    if parsed.args.contains(arg!(--help)) {
        println!("{}", parsed.help);
        return Ok(());
    }

    if parsed.args.contains(arg! { --write-default }) {
        config::write_default(plugins.default_configs())
            .await
            .map_err(|e| format!("Failed to write default config: {e}"))?;
        return Ok(());
    }

    let config_path = match parsed.args.get(arg!(--config)) {
        Some(path) => path.value().string(),
        None => "./config.toml",
    };

    config::open(config_path).await?;

    if parsed.args.contains(arg! { --create-user }) {
        auth::cli::create_user().await.map_err(|e| format!("Failed to create user: {e}"))?;
        return Ok(());
    }

    let default_level = log_filter(config!(logging.stdout_level))?;
    tiny_logs::init(
        default_level,
        config!(logging.file).clone(),
        config!(logging.file_level)
            .clone()
            .map(|f| log_filter(&f))
            .transpose()?
            .unwrap_or(default_level),
        #[cfg(feature = "syslog")]
        config!(logging.syslog_level)
            .clone()
            .map(|f| log_filter(&f))
            .transpose()?
            .unwrap_or(default_level),
    )
    .await
    .map_err(|e| format!("Failed to initialize logging: {e}"))?;

    let secret_key = {
        let path = config!(session_secret_key_path);
        fs::read(path)
            .await
            .map(Zeroizing::new)
            .map_err(|e| format!("Failed to read secret key file `{path}`: {e}"))?
    };
    if secret_key.len() < 64 {
        return Err("Session secret key must be 64 bytes long".into());
    }
    let secret_key = Key::from(&secret_key[..64]);

    let database = database::init().await.map_err(|e| format!("Failed to open database: {e}"))?;

    plugins
        .init(config!(plugins))
        .map_err(|e| format!("Failed to initialize plugins: {e}"))?;

    server::start(secret_key, database, plugins)
        .await
        .map_err(|e| format!("Server crashed: {e}"))
}
