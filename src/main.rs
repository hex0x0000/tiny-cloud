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

mod api;
mod auth;
mod config;
mod database;
mod error;
mod logging;
mod plugins;
mod server;
#[cfg(not(feature = "no-tls"))]
mod tls;
mod token;
mod utils;
mod webui;
#[macro_use]
mod macros;
use actix_web::cookie::Key;
use plugins::Plugins;
use tcloud_library::tiny_args::*;
use tokio::fs;
use zeroize::Zeroizing;

#[actix_web::main]
async fn main() {
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
    let cmd = cmd.build();

    let parsed = match cmd.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    if plugins.handle_args(&parsed) {
        return;
    }

    if parsed.args.get(arg!(--help)).is_some() {
        println!("{}", parsed.help);
        return;
    }

    if parsed.args.get(arg! { --write-default }).is_some() {
        if let Err(e) = config::write_default(plugins.default_configs()).await {
            eprintln!("Failed to write default config: {e}");
        }
        return;
    }

    let config_path = match parsed.args.get(arg!(--config)) {
        Some(path) => path.value().string(),
        None => "./config.toml".into(),
    };

    if let Err(e) = config::open(config_path).await {
        eprintln!("{e}");
        return;
    }

    if parsed.args.get(arg! { --create-user }).is_some() {
        if let Err(e) = auth::cli::create_user().await {
            eprintln!("Failed to create user: {e}");
        }
        return;
    }

    if let Err(e) = logging::init_logging() {
        eprintln!("Failed to initialize logging: {e}");
        return;
    }

    let secret_key = {
        let path = config!(session_secret_key_path);
        match fs::read(path).await {
            Ok(b) => Zeroizing::new(b),
            Err(e) => {
                log::error!("Failed to read secret key file `{path}`: {e}");
                return;
            }
        }
    };
    if secret_key.len() < 64 {
        log::error!("Session secret key must be 64 bytes long");
        return;
    }
    let secret_key = Key::from(&secret_key[..64]);

    let database = match database::init().await {
        Ok(db) => db,
        Err(e) => {
            log::error!("Failed to open database: {e}");
            return;
        }
    };

    if let Err(e) = plugins.init(config!(plugins)) {
        log::error!("Failed to initialize plugins: {e}");
        return;
    }

    if let Err(e) = server::start(secret_key, database, plugins).await {
        log::error!("Server crashed: {e}");
    }
}
