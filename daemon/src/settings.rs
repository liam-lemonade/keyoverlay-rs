extern crate anyhow;
extern crate config;

use config::Config;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::error;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Settings {
    config: Config,
    name: String,
}

pub fn make_config(web_port: u16, socket_port: u16, keys: String, reset: String) -> String {
    format!("{{\n\t\"web_port\": {web_port},\n\t\"socket_port\": {socket_port},\n\t\"keys\": {keys},\n\t\"reset\": {reset}\n}}")
}

impl Settings {
    // writes to a file with DEFAULT_FILE const
    fn create_default_config(name: &str) -> Result<Config> {
        let result = File::create(name);

        let mut file = result.with_context(|| "Failed to create default configuration file")?;

        file.write_all(
            make_config(
                7685,
                7686,
                "[ \"Z\", \"X\" ]".to_string(),
                "\"End\"".to_string(),
            )
            .as_bytes(),
        )
        .with_context(|| "Failed to write default configuration to file")?;

        Self::try_get_config(name)
    }

    pub fn replace(&self, json: &String) -> Result<()> {
        let mut stream = OpenOptions::new().write(true).open(&self.name)?;
        stream.write_all(json.as_bytes())?;
        stream.flush()?;

        Ok(())
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn try_get_config(name: &str) -> Result<Config> {
        // if the config file exists, load it
        if Path::new(name).exists() {
            let builder = Config::builder().add_source(config::File::with_name(name));

            let message = format!("Failed to get config! Deleting the file ({name}) and re-opening the program may fix this issue");
            builder.build().with_context(|| message)
        } else {
            // create config as it doesnt exist
            let message = format!("The configuration file could not be found. A default configuration ({name}) will be created.\n\nPlease read the github wiki (https://github.com/TheRacc2/keyoverlay-rs/wiki) to see configuration guides");
            error::messagebox(&message);

            Self::create_default_config(name)
        }
    }

    pub fn raw_json(&self) -> Result<String> {
        let buffer = std::fs::read_to_string(self.name.as_str())
            .with_context(|| "Failed to read config to string")?;

        Ok(buffer)
    }

    // <'de, T: serde::Deserialize<'de>> forces the type in T to be deserializable, and because config-rs
    // uses serde, it will guarantee a read so long as the config file is formatted correctly
    pub fn read_config<'de, T: serde::Deserialize<'de>>(&self, key: &str) -> Result<T> {
        let err = self.config.get::<T>(key).with_context(||
            format!(
                "Failed to read key \"{}\" from config. Deleting the file ({}) and re-opening the program may fix this issue",
                key,
                self.name
            )
        );

        Ok(err?)
    }

    pub fn new(name: &str) -> Result<Self> {
        Ok(Self {
            config: Self::try_get_config(name)?,
            name: name.to_string(),
        })
    }
}
