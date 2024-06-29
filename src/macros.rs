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
        use crate::config::{CONFIG, ERR_MSG};
        &CONFIG.get().expect(ERR_MSG)$(.$config)*
    }};
}
