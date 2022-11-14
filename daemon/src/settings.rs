extern crate config;

use config::Config;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::error;

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
    fn create_default_config(name: &str) -> Config {
        let result = File::create(name);

        let mut file = match result {
            Ok(f) => f,

            Err(error) => {
                error::handle_error("Failed to create default file.", error);
                error::shutdown(1);
            }
        };

        match file.write_all(DEFAULT_CONFIG) {
            Ok(_) => Self::try_get_config(name),

            Err(error) => {
                error::handle_error("Failed to write default config.", error);
                error::shutdown(1);
            }
        }
    }

    fn try_get_config(name: &str) -> Config {
        // if the config file exists, load it
        if Path::new(name).exists() {
            let builder = Config::builder().add_source(config::File::with_name(name));

            match builder.build() {
                Ok(config) => config,

                Err(error) => {
                    error::handle_error("Failed to get config! Deleting the file (settings.json) and re-opening the program may fix this issue.", error);
                    error::shutdown(1);
                }
            }
        } else {
            // create config as it doesnt exist
            let message = "The configuration file could not be found. A default configuration (settings.json) will be created.\n\nPlease read the github wiki (https://github.com/TheRacc2/keyoverlay/wiki) to see configuration guides.";
            error::messagebox(message);

            Self::create_default_config(name)
        }
    }

    // <'de, T: serde::Deserialize<'de>> forces the type in T to be deserializable, and because config-rs
    // uses serde, it will guarantee a read so long as the config file is formatted correctly
    pub fn read_config<'de, T: serde::Deserialize<'de>>(&self, key: &str) -> T {
        match self.config.get::<T>(key) {
            Ok(value) => value,

            Err(error) => {
                error::handle_error("Failed to read config! Deleting the file (settings.json) and re-opening the program may fix this issue.", error);
                error::shutdown(1);
            }
        } // no ; returns straight out of match statement
    }

    pub fn new(name: &str) -> Self {
        Self {
            config: Self::try_get_config(name),
        }
    }
}
