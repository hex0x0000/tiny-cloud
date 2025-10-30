// SPDX-License-Identifier: AGPL-3.0-or-later

/// Returns a new plugin instance.
/// Requires the feature's name and the plugin's specific type.
/// The plugin must implement a `new() -> Self` function.
#[macro_export]
macro_rules! plugin {
    ($plugin:ty) => {{
        let plugin = <$plugin>::new();
        (plugin.info().name.into(), Box::new(plugin) as Box<dyn Plugin>)
    }};
}
