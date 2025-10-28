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

#[cfg(doc)]
use crate::{ArgName, ArgValue};

/// A shorthand macro for [`ArgName`].
///
/// Using this macro to create [`ArgName`]s is equivalent to using its functions.
///
/// # Example
///
/// ```rust
/// # use tiny_args::*;
/// assert_eq!(ArgName::both('h', "help"), arg!(-'h', --help));
/// assert_eq!(ArgName::short('h'), arg!(-'h'));
/// assert_eq!(ArgName::long("help"), arg!(--help));
/// ```
///
/// You can also specify long arguments with multiple words separated by dashes.
/// When doing this, it is recommended to use curly brackets (`arg! { }`) to keep
/// rustfmt from formatting the arguments.
///
/// # Example
///
/// ```rust
/// # use tiny_args::*;
/// assert_eq!(arg! { --long-help }, ArgName::long("long-help"));
/// ```
#[macro_export]
macro_rules! arg {
    (--$long:ident) => {{
        ArgName::long_static(stringify!($long))
    }};

    (--$first:ident$(-$long:ident)+) => {{
        ArgName::long_static(concat!(stringify!($first), $("-", stringify!($long),)+))
    }};

    (-$short:literal, --$long:ident) => {{
        ArgName::both_static($short, stringify!($long))
    }};

    (-$short:literal, --$first:ident$(-$long:ident)+) => {{
        ArgName::both_static($short, concat!(stringify!($first), $("-", stringify!($long),)+))
    }};

    (-$short:literal) => {{
        ArgName::short($short)
    }};
}

/// Shorthand macro to specify [`ArgValue`]s during command's creation.
///
/// This macro is equivalent to using [`ArgValue`]'s constructor:
///
/// ```rust
/// # use tiny_args::*;
/// assert_eq!(value!(), ArgValue::Flag);
/// assert_eq!(value!(string), ArgValue::String(None));
/// assert_eq!(value!(path, "/default/path"), ArgValue::Path(Some("/default/path".into())));
/// ```
///
/// Accepted values are: `string`, `num`, `float`, `path`. Each corresponding to their [`ArgValue`]
/// field. Since [`ArgValue::Flag`] does not carry any value it is defined as `value!()`.
#[macro_export]
macro_rules! value {
    () => {
        ArgValue::Flag
    };
    (string) => {
        ArgValue::String(None)
    };
    (num) => {
        ArgValue::Num(None)
    };
    (float) => {
        ArgValue::Float(None)
    };
    (path) => {
        ArgValue::Path(None)
    };
    (string, $default:expr) => {
        ArgValue::String(Some($default.into()))
    };
    (num, $default:expr) => {
        ArgValue::Num(Some($default))
    };
    (float, $default:expr) => {
        ArgValue::Float(Some($default))
    };
    (path, $default:expr) => {
        ArgValue::Path(Some($default.into()))
    };
}
