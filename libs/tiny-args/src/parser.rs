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

fn args(mut argslist: ArgList, mut inputargs: Vec<String>) -> Result<ArgList, String> {
    let mut argnameopt: Option<ArgName> = None;
    while let Some(input) = inputargs.first() {
        if let Some(argname) = &argnameopt {
            argslist.init_arg(argname, &mut inputargs)?;
            argnameopt.take();
        } else if input.starts_with("--") {
            if let Some(input) = input.get(2..) {
                argnameopt.replace(ArgName::Long(input.into()));
                inputargs.remove(0);
            } else {
                return Err(format!("'{input}' is not a valid long argument."));
            }
        } else if input.starts_with('-') {
            if let Some(input) = input.chars().nth(1) {
                argnameopt.replace(ArgName::Short(input));
                inputargs.remove(0);
            } else {
                return Err(format!("'{input}' is not a valid short argument."));
            }
        } else {
            return Err(format!("'{input}' is not an argument nor a value."));
        }
    }
    if let Some(argname) = &argnameopt {
        argslist.init_arg(argname, &mut inputargs)?;
    }
    Ok(argslist)
}

// NOTE: use Vec extract_if when it becomes stable
fn extract(mut subcmds: Vec<Command>, name: &str) -> Option<Command> {
    let mut i = 0;
    while i < subcmds.len() {
        if subcmds[i].name == name {
            return Some(subcmds.remove(i));
        }
        i += 1;
    }
    None
}

fn traverse(root: Command, args: &mut Vec<String>) -> Result<Command, String> {
    let mut cmd = root;
    while let Some(arg) = args.first() {
        if arg.starts_with('-') {
            break;
        }
        if let Some(found) = extract(cmd.subcommands, arg) {
            cmd = found;
            args.remove(0);
        } else {
            return Err(format!("'{arg}' is not a valid subcommand."));
        }
    }
    Ok(cmd)
}

pub fn parse(root: Command, mut input: Vec<String>) -> Result<ParsedCommand, String> {
    input.remove(0);
    let command = traverse(root, &mut input)?;
    Ok(ParsedCommand {
        name: command.name,
        help: help::create(&command),
        args: args(command.args, input)?,
        parents: command.parents,
    })
}
