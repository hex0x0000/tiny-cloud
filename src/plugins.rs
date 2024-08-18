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

pub mod error;
mod macros;
use crate::*;
use actix_web::HttpResponse;
use api::plugins::FileForm;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{boxed::Box, sync::OnceLock};
use tcloud_library::plugin::{PluginInfo, User};
use tcloud_library::{plugin::Plugin, toml::Table, Json, Toml};

static PLUGIN_NAMES: OnceLock<Vec<&'static PluginInfo>> = OnceLock::new();

pub struct Plugins {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl Plugins {
    pub fn new() -> Self {
        let plugins = HashMap::from([plugin!("archive", tcloud_archive::ArchivePlugin)]);
        PLUGIN_NAMES
            .set(plugins.values().map(|p| p.info()).collect())
            .expect("Tried to initialize PLUGIN_NAMES while already initialized. This is a bug");
        Self { plugins }
    }

    pub fn add_subcmds<'a>(&self, mut cmd: CommandBuilder<&'a str>) -> CommandBuilder<&'a str> {
        for plugin in self.plugins.values() {
            if let Some(subcmd) = plugin.subcmd() {
                cmd = cmd.subcommand(subcmd);
            }
        }
        cmd
    }

    pub fn handle_args(&self, parsed: &ParsedCommand) -> bool {
        if !parsed.parents.is_empty() {
            if let Some(plugin) = self.plugins.get(&parsed.name) {
                plugin.handle_args(parsed);
                return true;
            }
        }
        false
    }

    pub fn default_configs(&self) -> Table {
        let mut table = Table::new();
        for (name, plugin) in &self.plugins {
            if let Some(config) = plugin.config() {
                table.insert(name.clone(), Toml::Table(config));
            }
        }
        table
    }

    pub fn init(&mut self, config: &Table) -> Result<(), String> {
        for (name, plugin) in &mut self.plugins {
            plugin.init(config.get(name))?;
            log::info!("Plugin '{name}' initialized.");
        }
        Ok(())
    }

    pub async fn request(&self, name: String, user: Option<User>, body: Json) -> HttpResponse {
        if let Some(plugin) = self.plugins.get(&name) {
            if plugin.info().admin_only {
                if !user.clone().map(|u| u.is_admin).unwrap_or(false) {
                    return HttpResponse::NotFound().body("");
                }
            }
            let path = plugin_path(&user, name);
            plugin.request(user, body, path).await
        } else {
            HttpResponse::NotFound().body("")
        }
    }

    pub async fn file(&self, name: String, user: Option<User>, file: FileForm) -> HttpResponse {
        if let Some(plugin) = self.plugins.get(&name) {
            if plugin.info().admin_only {
                if !user.clone().map(|u| u.is_admin).unwrap_or(false) {
                    return HttpResponse::NotFound().body("");
                }
            }
            let path = plugin_path(&user, name);
            plugin.file(user, file.file, file.info.into_inner(), path).await
        } else {
            HttpResponse::NotFound().body("")
        }
    }
}

fn plugin_path(user: &Option<User>, plugin: String) -> PathBuf {
    let mut path = PathBuf::from(config!(data_directory));
    match user {
        Some(user) => {
            path.push("users");
            path.push(&user.name);
            path.push(plugin);
        }
        None => {
            path.push("unauth");
            path.push(plugin);
        }
    };
    path
}

pub fn list() -> &'static Vec<&'static PluginInfo> {
    PLUGIN_NAMES
        .get()
        .expect("Tried to access PLUGIN_NAMES when this value was not initialized. This is a bug")
}
