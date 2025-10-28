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

/// Gets an image from assets
#[macro_export]
macro_rules! image {
    ($filename:expr) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", $filename))
    };
}

/// Gets an HTML/JS/CSS file from assets without escaping it
#[macro_export]
macro_rules! unescaped_webfile {
    ($filename:expr) => {
        include_str!(concat!(env!("OUT_DIR"), "/outdir/assets/webfiles/", $filename))
    };
}

/// Gets an HTML/JS/CSS file from assets
#[macro_export]
macro_rules! webfile {
    ($filename:expr) => {
        PreEscaped(include_str!(concat!(env!("OUT_DIR"), "/outdir/assets/webfiles/", $filename)))
    };
}
