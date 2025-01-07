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

use tcloud_library::actix_multipart::form::tempfile::TempFile;
use tcloud_library::actix_web::HttpResponse;
use tcloud_library::plugin::{PluginInfo, User, WebUI};
use tcloud_library::tiny_args::{arg, value, ArgName, ArgValue, Command, ParsedCommand};
use tcloud_library::toml::Table;
use tcloud_library::{async_trait, plugin::Plugin};
use tcloud_library::{Json, Toml};

#[derive(Debug)]
pub struct ArchivePlugin;

impl ArchivePlugin {
    pub fn new() -> Self {
        ArchivePlugin {}
    }
}

#[async_trait]
impl Plugin for ArchivePlugin {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "archive",
            source: env!("CARGO_PKG_REPOSITORY"),
            version: env!("CARGO_PKG_VERSION"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            admin_only: false,
        }
    }

    fn subcmd(&self) -> Option<Command> {
        Some(
            Command::create(self.info().name, env!("CARGO_PKG_DESCRIPTION"))
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .license(env!("CARGO_PKG_LICENSE"))
                .arg(arg!(-'h', --help), value!(), "Shows help for this plugin"),
        )
    }

    fn config(&self) -> Option<Table> {
        None
    }

    fn handle_args(&self, cmd: &ParsedCommand) {
        if cmd.args.count(arg!(--help)) > 0 || cmd.args.total_count() == 0 {
            println!("{}", cmd.help);
        }
    }

    fn init(&mut self, config: Option<&Toml>) -> Result<(), String> {
        Ok(())
    }

    async fn webui(&self) -> WebUI {
        WebUI {
            html: "".into(),
            js: "",
            css: "",
        }
    }

    async fn request(&self, user: Option<User>, body: Json, path: PathBuf) -> HttpResponse {
        todo!()
    }

    async fn file(
        &self,
        user: Option<User>,
        file: TempFile,
        info: Json,
        path: PathBuf,
    ) -> HttpResponse {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn init_test() {
        println!("{:?}", ArchivePlugin::new());
    }
}
