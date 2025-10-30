// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod error;
mod macros;
use crate::*;
use actix_web::HttpResponse;
use api::plugins::FileForm;
use common_library::plugin::{PluginInfo, User};
use common_library::{Json, Toml, plugin::Plugin, toml::Table};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{boxed::Box, sync::OnceLock};

static PLUGIN_NAMES: OnceLock<Vec<&'static PluginInfo>> = OnceLock::new();

pub struct Plugins {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl Plugins {
    pub fn new() -> Self {
        let plugins = HashMap::<String, Box<dyn Plugin>>::from([]);
        PLUGIN_NAMES
            .set(plugins.values().map(|p| p.info()).collect())
            .expect("Tried to initialize PLUGIN_NAMES while already initialized. This is a bug");
        Self { plugins }
    }

    pub fn add_subcmds(&self, mut cmd: Command) -> Command {
        for plugin in self.plugins.values() {
            if let Some(subcmd) = plugin.subcmd() {
                cmd = cmd.subcommand(subcmd);
            }
        }
        cmd
    }

    pub fn handle_args(&self, parsed: &ParsedCommand) -> bool {
        if !parsed.parents.is_empty() {
            if let Some(plugin) = self.plugins.get(parsed.name) {
                plugin.handle_args(parsed);
                return true;
            }
        }
        false
    }

    /// Default plugins' configs, will appear in the default config file
    pub fn default_configs(&self) -> Table {
        let mut table = Table::new();
        for (name, plugin) in &self.plugins {
            if let Some(config) = plugin.config() {
                table.insert(name.clone(), Toml::Table(config));
            }
        }
        table
    }

    /// Initializes all the plugins
    pub fn init(&mut self, config: &Table) -> Result<(), String> {
        for (name, plugin) in &mut self.plugins {
            plugin.init(config.get(name))?;
            log::info!("Plugin '{name}' initialized.");
        }
        Ok(())
    }

    pub async fn request(&self, name: String, user: Option<User>, body: Json) -> HttpResponse {
        if let Some(plugin) = self.plugins.get(&name) {
            if plugin.info().admin_only && !user.as_ref().map(|u| u.is_admin).unwrap_or(false) {
                return HttpResponse::NotFound().body("");
            }
            let path = plugin_path(&user, name);
            plugin.request(user, body, path).await
        } else {
            HttpResponse::NotFound().body("")
        }
    }

    pub async fn file(&self, name: String, user: Option<User>, file: FileForm) -> HttpResponse {
        if let Some(plugin) = self.plugins.get(&name) {
            if plugin.info().admin_only && !user.as_ref().map(|u| u.is_admin).unwrap_or(false) {
                return HttpResponse::NotFound().body("");
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
