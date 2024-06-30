/// Sets a plugin during initialization
#[macro_export]
macro_rules! set_plugin {
    ($plugin:ty) => {{
        let (name, plugin) = match <$plugin>::new() {
            Ok((name, plugin)) => (name, Box::new(plugin) as Box<dyn Plugin + Send + Sync>),
            Err(err) => panic!("`{}`: {}", stringify!($plugin), err),
        };
        (name, Mutex::new(plugin))
    }};
}

/// Gets a config's value
#[macro_export]
macro_rules! config {
    ($( $config:ident ).* ) => {{
        &crate::config::CONFIG.get().expect(crate::config::ERR_MSG)$(.$config)*
    }};
}
