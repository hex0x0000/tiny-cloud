// SPDX-License-Identifier: AGPL-3.0-or-later

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
