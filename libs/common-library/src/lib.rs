// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod error;
pub mod plugin;

pub use actix_multipart;
pub use actix_web;
pub use async_trait::async_trait;
pub use serde_json;
pub use tiny_args;
pub use toml;

/// Alias of [`serde_json::Value`] to avoid confusions with [`toml::Value`]
pub type Json = serde_json::Value;

/// Alias of [`toml::Value`] to avoid confusions with [`serde_json::Value`]
pub type Toml = toml::Value;

#[cfg(test)]
mod tests {}
