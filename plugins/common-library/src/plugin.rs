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

use std::path::PathBuf;

use crate::*;
use actix_multipart::form::tempfile::TempFile;
use actix_web::HttpResponse;
use serde::Serialize;

/// Struct containing Web UI.
pub struct WebUI {
    pub html: String,
    pub js: &'static str,
    pub css: &'static str,
}

/// Struct representing a user.
/// Contains username and its admin status.
#[derive(Clone)]
pub struct User {
    pub name: String,
    pub is_admin: bool,
}

/// Plugin information
#[derive(Serialize, Clone, Debug)]
pub struct PluginInfo {
    /// Name of the plugin
    pub name: &'static str,
    /// Repository of the plugin
    pub source: &'static str,
    /// Version of the plugin
    pub version: &'static str,
    /// Description of the plugin
    pub description: &'static str,
    /// Wether or not this plugin is only for admins
    pub admin_only: bool,
}

/// Plugin trait, every Tiny Cloud plugin must implement this trait.
/// It is used during plugin's initialization and when processing requests.
///
/// EVERY PLUGIN MUST IMPLEMENT `fn new() -> Self` IN ORDER TO INITIALIZE THE INSTANCE.
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Returns the information of this plugin
    fn info(&self) -> &'static PluginInfo;

    /// Returns the commands of the plugin.
    /// See the [`tiny_args`] crate to see how to make a commandline.
    /// If you don't want to implement any sub command simply return [`None`].
    ///
    /// THE SUB COMMAND'S NAME MUST BE THE SAME NAME RETURNED BY [`Plugin::name`].
    fn subcmd(&self) -> Option<tiny_args::Command>;

    /// Returns the default configuration of the plugin.
    fn config(&self) -> Option<toml::Table>;

    /// Handles command line arguments. After that the server exits.
    ///
    /// - `cmd`: Command parsed from the command line.
    fn handle_args(&self, cmd: &tiny_args::ParsedCommand);

    /// Initializes the plugin with its configuration.
    /// Returns an error message if something went wrong.
    ///
    /// - `config`: Parsed configuration of this plugin.
    fn init(&mut self, config: Option<&Toml>) -> Result<(), String>;

    /// Returns the webui of this plugin.
    async fn webui(&self) -> WebUI;

    /// Processes a request.
    ///
    /// - `user`: The user that made this request, [`None`] if it is unrecognized/unregistered.
    /// - `body`: Body of the request as [`Json`].
    /// - `path`: Path where the plugin can manage its files for the specific user.
    async fn request(&self, user: Option<User>, body: Json, path: PathBuf) -> HttpResponse;

    /// Processes a file upload.
    ///
    /// - `file`: Temporary file where the uploaded file has been stored.
    /// - `info`: Info sent along with the file.
    /// - `path`: Path where the plugin can manage its files for the specific user.
    async fn file(
        &self,
        user: Option<User>,
        file: TempFile,
        info: Json,
        path: PathBuf,
    ) -> HttpResponse;
}

#[cfg(test)]
mod tests {}
