//#![allow(dead_code)]

extern crate toml;
extern crate const_format;

mod delegates;
mod error;
mod helper;
mod settings;

use std::thread;

use anyhow::Context;
use const_format::formatcp;

use settings::{OverlaySettings, Settings};
use error::ErrorStatus;

static SETTINGS_FILENAME: &str = "settings.toml";

pub const NAME: &str = "keyoverlay-rs";
pub const BUILD: &str = "oxide";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const TITLE: &str = formatcp!("{NAME} ({BUILD} v{VERSION})");

macro_rules! start_delegate {
    ($delegate_name:ident, $settings:expr) => {
        println!("starting {} thread", stringify!($delegate_name));

        if let Err(error) = delegates::$delegate_name::start($settings) {
            error::display_error(stringify!($delegate_name), error);
        }
    };
}

fn load_configuration() -> anyhow::Result<OverlaySettings> {
    let mut toml_settings = Settings::default();

    if helper::is_first_run(SETTINGS_FILENAME) {
        let message = 
            formatcp!("{}\n\n{}",
                "This appears to be the first time you've run the program, and no configuration file currently exists.",
                "Press \"OK\" to create a default configuration now."
            );

        error::display_message(message, false);

        helper::create_default_file(SETTINGS_FILENAME, toml_settings.clone())?;
    } else {
        // load configuration
        let settings_string = std::fs::read_to_string(SETTINGS_FILENAME)
            .with_context(|| "Failed to read from configuration file")?;

        toml_settings = toml::from_str::<Settings>(&settings_string)
            .with_context(|| "Failed to deserialize settings")?;
    }

    OverlaySettings::from_toml(toml_settings)
}

fn start_delegates(settings: OverlaySettings) {
    let server_settings = settings.clone();
    thread::spawn(move || {
        start_delegate!(server, server_settings);
    });

    let keyboard_settings = settings.clone();
    thread::spawn(move || {
        start_delegate!(keyboard, keyboard_settings);
    });

    start_delegate!(gui, settings);
}

fn main() {
    println!("{} started", TITLE);

    match load_configuration() {
        Ok(settings) => {
            start_delegates(settings);
        }

        Err(error) => {
            error::display_error("main", error);
            error::shutdown(ErrorStatus::FAILURE);
        }
    }
}
