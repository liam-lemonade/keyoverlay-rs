#![cfg_attr(
    all(target_os = "windows", not(feature = "debug"),),
    windows_subsystem = "windows"
)]

extern crate anyhow;
extern crate lazy_static;
extern crate serde;
extern crate toml;

mod error;
mod gui;
mod keyboard;
mod server;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    sync::RwLock,
    thread,
};

use anyhow::{Context, Result};

use error::ErrorCode;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub const CONFIG_NAME: &str = "settings.toml";

#[derive(Serialize, Deserialize, Clone)]
struct Settings {
    static_path: String,

    keyboard: Keyboard,

    server: Server,
}

#[derive(Serialize, Deserialize, Clone)]
struct Keyboard {
    keys: Vec<String>,
    reset: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Server {
    ip: String,
    port: u16,
}

macro_rules! read_settings {
    () => {
        SETTINGS.read().unwrap()
    };
}

macro_rules! write_settings {
    () => {
        SETTINGS.write().unwrap()
    };
}

lazy_static! {
    static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings {
        static_path: "static".to_string(),

        keyboard: Keyboard {
            keys: vec!["Z".to_string(), "X".to_string()],
            reset: "End".to_string(),
        },

        server: Server {
            ip: "127.0.0.1".to_string(),
            port: 7686
        },
    });
}

fn main() -> Result<()> {
    if let Err(error) = || -> Result<()> {
        // populate SETTINGS variable
        if Path::new(CONFIG_NAME).exists() {
            // load file
            let toml_settings =
                fs::read_to_string(CONFIG_NAME).with_context(|| "Failed to read settings file")?;

            *write_settings!() = toml::from_str(toml_settings.as_str())
                .with_context(|| "Failed to deserialize settings")?;
        } else {
            // create file and write
            let mut file = File::create(CONFIG_NAME)
                .with_context(|| format!("Failed to create file: \"{CONFIG_NAME}\""))?;

            let settings = read_settings!();
            let toml_settings = toml::to_string_pretty(&*settings)
                .with_context(|| "Failed to serialize settings")?;

            file.write(toml_settings.as_bytes())
                .with_context(|| "Failed to write settings to file")?;
        }

        let web_path = if cfg!(feature = "debug") {
            "D:\\code\\rust\\keyoverlay-rs\\static".to_string()
        } else {
            read_settings!().static_path.clone()
        };

        // start threads
        thread::spawn(|| {
            if let Err(err) = keyboard::start() {
                error::msgbox("An error occurred while running the keyboard thread", err);
                error::shutdown(ErrorCode::Failure);
            }
        });

        thread::spawn(|| {
            let ip;
            let port;
            {
                let settings = read_settings!();

                ip = settings.server.ip.clone();
                port = settings.server.port;
            }

            let address = format!("{}:{}", ip, port);
            if let Err(err) = server::start(web_path, address) {
                error::msgbox("An error occurred while running the server thread", err);
                error::shutdown(ErrorCode::Failure);
            }
        });

        if let Err(err) = gui::start() {
            error::msgbox("An error occurred while running the gui thread", err);
            error::shutdown(ErrorCode::Failure);
        }

        Ok(())
    }() {
        error::msgbox("An error occurred while running the main thread", error);
        error::shutdown(ErrorCode::Failure);
    }

    error::shutdown(error::ErrorCode::Success)
}
