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

//! # What is this?
//!
//! This is a bare-bones parser for CLI commands made for [Tiny Cloud](https://github.com/personal-tiny-cloud/tiny-cloud).
//! It was made in place of [clap](https://docs.rs/clap/latest/clap/) because Tiny Cloud
//! needs a way to configure and execute subcommands from different crates.
//! This crate can be used for other projects too, but keep it mind that it was made for a specific project.
//! If you need some particular features that are not supposed to be here you should use some other crate.
//!
//! # Example
//!
//! ```rust
//! use tiny_args::*;
//!
//! let parsed = Command::create("myapp", "This is my cool app!")
//!         .author("Me!")
//!         .version("0.1.0")
//!         .license("SOME-LICENSE")
//!         .arg(arg!(-'h', --help), value!(), "Shows this help.")
//!         .arg(arg! { -'s', --some-words }, value!(string), "Inserts some words.")
//!         .arg(arg!(--path), value!(path, "/default/path"), "Specify a path to something.")
//!         .subcommand(
//!             Command::create("subcmd", "This is a subcommand.")
//!                 .arg(arg!(-'h', --help), value!(), "Shows this help.")
//!                 .arg(arg!(-'n', --num), value!(num, 42), "Insert a number.")
//!         )
//!         .parse()
//!         .unwrap(); // Show the error to the user instead of panicking!!!
//!
//! if parsed.args.count(arg!(-'h')) > 0 {
//!     println!("{}", parsed.help);
//!     return;
//! }
//!
//! // Safe to unwrap since it has a default value.
//! let path = parsed.args.get(arg!(--path)).path().unwrap();
//! println!("Path to something: {}", path.display());
//!
//! if let Some(words) = parsed.args.get(arg!(-'s')).string() {
//!     println!("Your words: {words}");
//! }
//! ```

#![warn(missing_docs)]

use std::{env, fmt, path::PathBuf};

use smol_str::SmolStr;

mod help;
mod parser;
#[macro_use]
mod macros;
#[cfg(test)]
mod tests;

/// The argument's values.
///
/// This enum is used during the initialization of the command to specify the argument's value type
/// and a default value. (Use [`None`] to set no default value)
///
/// You can either use the shorthand macro [`value`] or the enum's constructor to create it.
#[derive(Clone, Debug, PartialEq)]
pub enum ArgValue {
    /// Carries a [`String`]
    String(Option<String>),

    /// Carries an [`i64`]
    Num(Option<i64>),

    /// Carries a [`f64`]
    Float(Option<f64>),

    /// Carries a [`PathBuf`]
    Path(Option<PathBuf>),

    /// Flags do not carry any value.
    Flag,
}

/// Name of an argument. It contains both short and/or long names of the argument.
///
/// You can either use this enum's function or its shorthand macro [`arg`] to initialize it.
/// This macro contains both short and/or long names of the argument. Two arguments with the
/// same name will be treated as equal.
///
/// For example:
///
/// ```rust
/// # use tiny_args::*;
/// assert_eq!(arg! { -'h', --help }, arg!(-'h'));
/// assert_eq!(arg! { -'h', --help }, arg!(--help));
/// ```
#[derive(Eq, Clone, Debug)]
pub enum ArgName {
    /// Represents a short argument.
    ///
    /// It is formed by a dash and a character on the command line (e.g. `-h`).
    /// The dash is omitted in the enum's value.
    /// When turned into a string this enum recreates the argument.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tiny_args::ArgName;
    /// assert_eq!(ArgName::short('h').to_string(), "-h");
    /// ```
    Short(char),

    /// Represents a long argument.
    ///
    /// It is formed by two dashes and a string on the command line (e.g. `--help`).
    /// The dashes are omitted in the enum's value.
    /// When turned into a string this enum recreates the argument.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tiny_args::ArgName;
    /// assert_eq!(ArgName::long("help").to_string(), "--help");
    /// ```
    Long(SmolStr),

    /// Represents both a long and a short argument.
    ///
    /// When turned into a string this enum prints both arguments.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tiny_args::ArgName;
    /// assert_eq!(ArgName::both('h', "help").to_string(), "-h, --help");
    /// ```
    Both {
        /// Short argument's name.
        short: char,

        /// Long argument's name.
        long: SmolStr,
    },
}

impl ArgName {
    /// Creates a new short [`ArgName`] from a char.
    #[inline(always)]
    pub fn short(name: char) -> Self {
        Self::Short(name)
    }

    /// Creates a new long [`ArgName`].
    #[inline(always)]
    pub fn long(name: &str) -> Self {
        Self::Long(SmolStr::from(name))
    }

    /// Creates a new long [`ArgName`] from a static string.
    ///
    /// Consider using the [`arg`] macro instead of this function.
    #[inline(always)]
    pub fn long_static(name: &'static str) -> Self {
        Self::Long(SmolStr::new_static(name))
    }

    /// Creates a new [`ArgName`] with short and long options.
    #[inline(always)]
    pub fn both(short: char, long: &str) -> Self {
        Self::Both {
            short,
            long: SmolStr::new(long),
        }
    }

    /// Creates a new [`ArgName`] with short and long options from a static strings.
    ///
    /// Consider using the [`arg`] macro instead of this function.
    #[inline(always)]
    pub fn both_static(short: char, long: &'static str) -> Self {
        Self::Both {
            short,
            long: SmolStr::new_static(long),
        }
    }
}

impl PartialEq for ArgName {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Self::Short(s) => match *other {
                Self::Short(o) => *s == o,
                Self::Long(_) => false,
                Self::Both { short, .. } => *s == short,
            },
            Self::Long(s) => match &other {
                Self::Short(_) => false,
                Self::Long(o) => *s == *o,
                Self::Both { long, .. } => *s == *long,
            },
            Self::Both { short, long } => match &other {
                Self::Short(o) => *short == *o,
                Self::Long(o) => *long == *o,
                Self::Both { short: s, long: l } => *short == *s || *long == *l,
            },
        }
    }
}

impl fmt::Display for ArgName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Short(s) => write!(f, "-{s}"),
            Self::Long(l) => write!(f, "--{l}"),
            Self::Both { short, long } => {
                write!(f, "-{short}, --{long}")
            }
        }
    }
}

/// A struct containing all the information of an argument.
///
/// This struct is not available until [`Command`] has been parsed.
#[non_exhaustive]
#[derive(Clone)]
pub struct Arg {
    /// Name of this argument.
    pub argname: ArgName,

    /// Value of this argument.
    ///
    /// If no default value was specified the internal value is [`None`].
    pub argvalue: ArgValue,

    /// Description of this argument
    pub description: &'static str,

    /// How many time this argument was called in the command line. (`0` if none)
    ///
    /// Note: arguments can be called multiple times, but if they carry a value only the last one
    /// is saved. A counter is usually useful for some types of flags, or to check if the
    /// argument was called in command line, instead of containing just the default value.
    pub counter: usize,
}

impl Arg {
    fn new(argname: ArgName, argvalue: ArgValue, description: &'static str) -> Self {
        Self {
            argname,
            argvalue,
            description,
            counter: 0, // Counts how many times the argument has been called.
        }
    }

    /// Returns the [`String`] value of the argument.
    ///
    /// If no value (not even default) was specified or if it is not an [`ArgValue::String`]
    /// it returns [`None`].
    pub fn string(&self) -> Option<&str> {
        if let ArgValue::String(Some(value)) = &self.argvalue {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the [`i64`] value of the argument.
    ///
    /// If no value (not even default) was specified or if it is not an [`ArgValue::Num`]
    /// it returns [`None`].
    pub fn num(&self) -> Option<i64> {
        if let ArgValue::Num(Some(value)) = self.argvalue {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the [`f64`] value of the argument.
    ///
    /// If no value (not even default) was specified or if it is not an [`ArgValue::Float`]
    /// it returns [`None`].
    pub fn float(&self) -> Option<f64> {
        if let ArgValue::Float(Some(value)) = self.argvalue {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the [`PathBuf`] value of the argument.
    ///
    /// If no value (not even default) was specified or if it is not an [`ArgValue::String`]
    /// it returns [`None`].
    pub fn path(&self) -> Option<&PathBuf> {
        if let ArgValue::Path(Some(value)) = &self.argvalue {
            Some(value)
        } else {
            None
        }
    }

    fn init(&mut self, input: &mut Vec<String>) -> Result<(), String> {
        match self.argvalue {
            ArgValue::String(_) => self.argvalue = ArgValue::String(Some(input.remove(0))),
            ArgValue::Num(_) => {
                self.argvalue = ArgValue::Num(Some(input.remove(0).parse().map_err(|e| {
                    format!("'{}' value's must be a valid number: {e}", self.argname)
                })?))
            }
            ArgValue::Float(_) => {
                self.argvalue = ArgValue::Float(Some(input.remove(0).parse().map_err(|e| {
                    format!(
                        "'{}' value's must be a valid float number: {e}",
                        self.argname
                    )
                })?))
            }
            ArgValue::Path(_) => {
                self.argvalue = ArgValue::Path(Some(PathBuf::from(input.remove(0))))
            }
            ArgValue::Flag => (),
        }
        self.counter += 1;
        Ok(())
    }
}

/// A list of arguments.
///
/// This list is accessible only after the command line arguments have been parsed.
/// You can access specific arguments with [`ArgList::get`].
#[repr(transparent)]
pub struct ArgList {
    args: Vec<Arg>,
}

impl ArgList {
    fn new() -> Self {
        ArgList { args: Vec::new() }
    }

    fn insert(&mut self, arg: Arg) {
        if self.args.iter().any(|a| a.argname == arg.argname) {
            panic!(
                "The argument '{}' already exists in this command",
                arg.argname
            );
        }
        self.args.push(arg);
    }

    /// Returns the inner [`Vec`] with parsed [`Arg`]s.
    pub fn inner(&self) -> &Vec<Arg> {
        &self.args
    }

    /// Returns a given argument [`Arg`] by its [`ArgName`].
    ///
    /// # Panics
    ///
    /// Panics if the given argument does not exist in the [`Command`].
    pub fn get(&self, argname: ArgName) -> &Arg {
        self.args
            .iter()
            .find(|&arg| arg.argname == argname)
            .unwrap_or_else(|| panic!("Argument '{argname}' does not exist"))
    }

    /// Returns a given argument [`Arg`] by its [`ArgName`].
    ///
    /// Does not panic but returns [`None`] if the argument does not exist in the [`Command`].
    pub fn try_get(&self, argname: ArgName) -> Option<&Arg> {
        self.args.iter().find(|&arg| arg.argname == argname)
    }

    /// Counts how many arguments were inserted by the user.
    pub fn total_count(&self) -> usize {
        let mut count: usize = 0;
        for arg in &self.args {
            count += arg.counter;
        }
        count
    }

    /// Checks how many times the argument has been inserted (`0` if none). Usually useful for flags.
    ///
    /// # Panics
    ///
    /// Panics if the given `argname` does not exist in the [`Command`].
    pub fn count(&self, argname: ArgName) -> usize {
        self.args
            .iter()
            .find(|arg| arg.argname == argname)
            .map(|arg| arg.counter)
            .unwrap_or_else(|| panic!("Flag '{argname}' does not exist"))
    }

    fn init_arg(&mut self, argname: &ArgName, input: &mut Vec<String>) -> Result<(), String> {
        for arg in &mut self.args {
            if arg.argname == *argname {
                arg.init(input)?;
                return Ok(());
            }
        }
        Err(format!("'{argname}' is not a valid argument."))
    }
}

/// Builds the command line.
///
/// It can be then used to parse the command line to get the arguments inserted by the user.
/// See [`Command::parse`] or [`Command::parse_from`].
///
/// # Example
///
/// ```rust
/// # use tiny_args::*;
/// let parsed = Command::create("myapp", "This is my cool app.")
///     .author("Me!")
///     .license("MY-LICENSE")
///     .version("0.1.0")
///     .arg(arg!(-'h', --help), value!(), "Shows this help.")
///     .subcommand(
///         Command::create("subcmd", "This is a subcommand.")
///             .arg(arg!(-'p', --path), value!(path), "Insert a path.")
///     )
///     .parse();
/// ```
pub struct Command {
    name: &'static str,
    description: &'static str,
    author: Option<&'static str>,
    version: Option<&'static str>,
    license: Option<&'static str>,
    color: bool,
    args: ArgList,
    subcommands: Vec<Command>,
    parents: Vec<&'static str>,
}

impl Command {
    /// This function creates a new [`Command`].
    pub fn create(name: &'static str, description: &'static str) -> Self {
        Command {
            name,
            description,
            version: None,
            author: None,
            license: None,
            args: ArgList::new(),
            subcommands: Vec::new(),
            parents: Vec::new(),
            color: true,
        }
    }

    /// Specifies a new argument.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use tiny_args::*;
    /// let cmd = Command::create("myapp", "This is my cool app.")
    ///     .arg(arg!(-'h', --help), value!(), "Shows this help.");
    /// ```
    ///
    /// # Panic
    ///
    /// Panics if an argument with the same name was already inputted.
    #[inline]
    pub fn arg(mut self, argname: ArgName, argtype: ArgValue, description: &'static str) -> Self {
        self.args.insert(Arg::new(argname, argtype, description));
        self
    }

    /// Specifies a new subcommand [`Command`].
    ///
    /// # Panic
    ///
    /// Panics if a subcommand with the same name was already inputted.
    pub fn subcommand(mut self, subcmd: Command) -> Self {
        if self.subcommands.iter().any(|s| s.name == subcmd.name) {
            panic!("Subcommand '{}' already exists.", subcmd.name);
        }
        let mut subcmd = subcmd;
        subcmd.add_parents(self.parents.clone(), self.name);
        self.subcommands.push(subcmd);
        self
    }

    /// Specifies the version of the program.
    ///
    /// It will appear on the help page ([`ParsedCommand::help`]).
    #[inline]
    pub fn version(mut self, version: &'static str) -> Self {
        self.version = Some(version);
        self
    }

    /// Specifies the author of the program.
    ///
    /// It will appear on the help page ([`ParsedCommand::help`]).
    #[inline]
    pub fn author(mut self, author: &'static str) -> Self {
        self.author = Some(author);
        self
    }

    /// Specifies the license of the program.
    ///
    /// It will appear on the help page ([`ParsedCommand::help`]).
    #[inline]
    pub fn license(mut self, license: &'static str) -> Self {
        self.license = Some(license);
        self
    }

    /// Specifies whether or not the help page should be colored.
    /// By default it is colored.
    #[inline]
    pub fn color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    fn add_parents(&mut self, grandparents: Vec<&'static str>, parent: &'static str) {
        let mut parents = grandparents;
        parents.push(parent);
        self.parents = parents;
    }

    /// Parses the command line arguments given by [`env::args`].
    ///
    /// # Returns
    ///
    /// This function returns a [`Result`] that contains the [`ParsedCommand`].
    /// In case of error, a [`String`] will be returned containing an error message that can be
    /// displayed to the user.
    #[inline]
    pub fn parse(self) -> Result<ParsedCommand, String> {
        self.parse_from(env::args().collect())
    }

    /// Parses command line arguments from a custom [`Vec<String>`] list of arguments.
    ///
    /// # Returns
    ///
    /// This function returns a [`Result`] that contains the [`ParsedCommand`].
    /// In case of error, a [`String`] will be returned containing an error message that can be
    /// displayed to the user.
    #[inline]
    pub fn parse_from(self, args: Vec<String>) -> Result<ParsedCommand, String> {
        parser::parse(self, args)
    }
}

/// A struct representing a parsed command.
#[non_exhaustive]
pub struct ParsedCommand {
    /// Name of the command or subcommand.
    pub name: &'static str,

    /// The help page of the parsed command.
    ///
    /// It can be displayed to the user, for example when the `--help` flag is used.
    pub help: String,

    /// The list of parsed arguments.
    ///
    /// You can access the values of each argument value inputted by the user.
    pub args: ArgList,

    /// The parent commands if this is a subcommand.
    ///
    /// If this is the root of the program the [`Vec`] is empty.
    pub parents: Vec<&'static str>,
}
