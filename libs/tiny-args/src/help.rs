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

use crate::*;
use owo_colors::OwoColorize;

fn tabs(len: usize) -> &'static str {
    match len / 8 {
        0 => "\t\t",
        1 => "\t",
        _ => "\n\t\t\t",
    }
}

fn license(cmd: &Command) -> String {
    if let Some(license) = &cmd.license {
        format!("Licensed under {license}")
    } else {
        "".into()
    }
}

fn subcommands_normal(cmd: &Command) -> String {
    if cmd.subcommands.is_empty() {
        return "".into();
    }
    let mut buf = String::from("SUBCOMMANDS:\n");
    for subcmd in &cmd.subcommands {
        buf.push_str(&format!(
            "\t{name}{tabs}{description}\n",
            name = subcmd.name,
            description = subcmd.description,
            tabs = tabs(subcmd.name.len())
        ));
    }
    buf.push('\n');
    buf
}

fn args_normal(cmd: &Command) -> String {
    if cmd.args.args.is_empty() {
        return "".into();
    }
    let mut buf = String::from("ARGS:\n");
    for arg in &cmd.args.args {
        let name = arg.argname.to_string();
        buf.push_str(&format!(
            "\t{name}{tabs}{description}\n",
            description = arg.description,
            tabs = tabs(name.len())
        ));
    }
    buf
}

fn usage_normal(cmd: &Command, fullname: &str) -> String {
    let mut buf = String::from("USAGE:");
    if !cmd.args.args.is_empty() {
        buf.push_str(&format!("\n\t{fullname} [ARGS]"))
    }
    if !cmd.subcommands.is_empty() {
        buf.push_str(&format!("\n\t{fullname} [SUBCOMMAND] [ARGS]"))
    }
    buf
}

fn create_normal(cmd: &Command) -> String {
    let fullname = format!("{} {}", cmd.parents.join(" "), cmd.name);
    let fullname = fullname.trim();
    format!(
        "{fullname} {version}
{author}{description}

{usage}

{args}
{subcommands}{license}",
        fullname = fullname,
        description = cmd.description,
        version = cmd.version.unwrap_or(""),
        author = cmd.author.map(|a| format!("{a}\n")).unwrap_or("".into()),
        usage = usage_normal(cmd, fullname),
        args = args_normal(cmd),
        subcommands = subcommands_normal(cmd),
        license = license(cmd)
    )
}

fn subcommands_color(cmd: &Command) -> String {
    if cmd.subcommands.is_empty() {
        return "".into();
    }
    let mut buf: String = format!("{}", "SUBCOMMANDS:\n".bold().underline());
    for subcmd in &cmd.subcommands {
        buf.push_str(&format!(
            "\t{name}{tabs}{description}\n",
            name = subcmd.name.bold(),
            description = subcmd.description,
            tabs = tabs(subcmd.name.len())
        ));
    }
    buf.push('\n');
    buf
}

fn args_color(cmd: &Command) -> String {
    if cmd.args.args.is_empty() {
        return "".into();
    }
    let mut buf: String = format!("{}", "ARGS:\n".bold().underline());
    for arg in &cmd.args.args {
        let name = arg.argname.to_string();
        buf.push_str(&format!(
            "\t{name}{tabs}{description}\n",
            name = name.bold(),
            description = arg.description,
            tabs = tabs(name.len())
        ));
    }
    buf
}

fn usage_color(cmd: &Command, fullname: &str) -> String {
    let mut buf: String = format!("{}", "USAGE:".bold().underline());
    if !cmd.args.args.is_empty() {
        buf.push_str(&format!(
            "\n\t{fullname} [ARGS]",
            fullname = fullname.bold()
        ))
    }
    if !cmd.subcommands.is_empty() {
        buf.push_str(&format!(
            "\n\t{fullname} [SUBCOMMAND] [ARGS]",
            fullname = fullname.bold()
        ))
    }
    buf
}

fn create_color(cmd: &Command) -> String {
    let fullname = format!("{} {}", cmd.parents.join(" "), cmd.name);
    let fullname = fullname.trim();
    format!(
        "{fullname} {version}
{author}{description}

{usage}

{args}
{subcommands}{license}",
        fullname = fullname.bold(),
        description = cmd.description,
        version = cmd.version.unwrap_or("").dimmed(),
        author = cmd
            .author
            .map(|a| format!("{a}\n"))
            .unwrap_or("".into())
            .italic(),
        usage = usage_color(cmd, fullname),
        args = args_color(cmd),
        subcommands = subcommands_color(cmd),
        license = license(cmd).bold()
    )
}

pub fn create(cmd: &Command) -> String {
    if cmd.color {
        create_color(cmd)
    } else {
        create_normal(cmd)
    }
}
