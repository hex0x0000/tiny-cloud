// SPDX-License-Identifier: AGPL-3.0-or-later

/// Gets a config's value
#[macro_export]
macro_rules! config {
    ($( $config:ident ).* ) => {{
        &crate::config::get()$(.$config)*
    }};
}
