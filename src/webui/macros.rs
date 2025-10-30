// SPDX-License-Identifier: AGPL-3.0-or-later

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
