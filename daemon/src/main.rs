#![cfg_attr(
    all(target_os = "windows", not(feature = "console"),),
    windows_subsystem = "windows"
)]

extern crate anyhow;

use settings::Settings;

use error::ExitStatus;
use std::{sync::mpsc, thread};

pub mod error;
pub mod gui;
pub mod keyboard;
pub mod server;
pub mod settings;

fn main() -> anyhow::Result<()> {
    let settings = Settings::new("settings.json").unwrap_or_else(|error| {
        error::handle_error(
            "An error occured while attempting to get the configuration",
            error,
        );

        error::shutdown(ExitStatus::Failure);
    });

    let socket_server_settings = settings.clone();
    thread::spawn(move || {
        if let Err(error) = server::spawn_socket_server(socket_server_settings) {
            error::handle_error(
                "An error occured while running the socket server thread",
                error,
            );
            error::shutdown(ExitStatus::Failure);
        }
    });

    let keyboard_settings = settings.clone();
    thread::spawn(move || {
        if let Err(error) = keyboard::hook_keyboard(keyboard_settings) {
            error::handle_error("An error occured while running the keyboard thread", error);
            error::shutdown(ExitStatus::Failure);
        }
    });

    let server_settings = settings.clone();
    thread::spawn(move || {
        if let Err(error) = server::spawn_webserver(server_settings) {
            error::handle_error("An error occured while running the webserver thread", error);
            error::shutdown(ExitStatus::Failure);
        }
    });

    if let Err(error) = gui::start_gui(settings) {
        error::handle_error("An error occured while running the gui thread", error);
        error::shutdown(ExitStatus::Failure);
    }

    error::shutdown(ExitStatus::Success);
}
