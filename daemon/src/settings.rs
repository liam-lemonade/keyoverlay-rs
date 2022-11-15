extern crate anyhow;
extern crate config;

use config::Config;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::error;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Settings {
    config: Config,
}

const DEFAULT_CONFIG: &[u8] = b"{
    \"web_port\": 7685,
    \"socket_port\": 7686,
    \"keys\": [ \"Z\", \"X\" ],
    \"reset\": \"End\"
}";

impl Settings {
    // writes to a file with DEFAULT_FILE const
    fn create_default_config(name: &str) -> Result<Config> {
        let result = File::create(name);

        let mut file = result.with_context(|| "Failed to create default configuration file")?;

        file.write_all(DEFAULT_CONFIG)
            .with_context(|| "Failed to write default configuration to file")?;

        Self::try_get_config(name)
    }

    fn try_get_config(name: &str) -> Result<Config> {
        // if the config file exists, load it
        if Path::new(name).exists() {
            let builder = Config::builder().add_source(config::File::with_name(name));

            let message = "Failed to get config! Deleting the file (settings.json) and re-opening the program may fix this issue.";
            builder.build().with_context(|| message)
        } else {
            // create config as it doesnt exist
            let message = "The configuration file could not be found. A default configuration (settings.json) will be created.\n\nPlease read the github wiki (https://github.com/TheRacc2/keyoverlay-rs/wiki) to see configuration guides.";
            error::messagebox(message);

            Self::create_default_config(name)
        }
    }

    // <'de, T: serde::Deserialize<'de>> forces the type in T to be deserializable, and because config-rs
    // uses serde, it will guarantee a read so long as the config file is formatted correctly
    pub fn read_config<'de, T: serde::Deserialize<'de>>(&self, key: &str) -> Result<T> {
        let err = self.config.get::<T>(key).with_context(||
            format!(
                "Failed to read key \"{}\" from config. Deleting \"settings.json\" and re-opening the program may fix this issue",
                key
            )
        );

        Ok(err?)
    }

    pub fn new(name: &str) -> Result<Self> {
        Ok(Self {
            config: Self::try_get_config(name)?,
        })
    }
}
