use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

pub fn get_configuration<'de, T: Deserialize<'de>>(path: &str) -> Result<T, ConfigError> {
    let run_mode = env::var("ENV").unwrap_or_else(|_| "development".into());

    let filename = File::with_name(&format!("{path}/config/default.toml")).required(false);
    println!("filename: {:?}", filename);
    println!("env directory: {:?}", env::current_dir().unwrap());

    let s = Config::builder()
        // Start off by merging in the "default" configuration file
        .add_source(File::with_name(&format!("{path}/config/default.toml")))
        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        .add_source(File::with_name(&format!("{path}/config/{run_mode}.toml")).required(false))
        // Add in a local configuration file
        // This file shouldn't be checked in to git
        .add_source(File::with_name(&format!("{path}/config/local.toml")).required(false))
        // Add in settings from the environment
        .add_source(Environment::default())
        .build()?;

    println!("debug: {:?}", s.get_bool("debug"));

    //freezing the entire configuration as

    //tracing::debug!("Settings values: {:?}", &settings);

    s.try_deserialize::<T>()
}
