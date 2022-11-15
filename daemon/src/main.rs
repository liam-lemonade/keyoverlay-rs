#![cfg_attr(
    all(target_os = "windows", not(feature = "console"),),
    windows_subsystem = "windows"
)]

extern crate tray_item;

pub mod error;
pub mod keyboard;
pub mod server;
pub mod settings;

use settings::Settings;

use tray_item::TrayItem;

use std::sync::mpsc;
use std::thread;

fn spawn_tray(settings: Settings) {
    let result = TrayItem::new("KeyOverlay Daemon", "keyoverlay-icon");

    let mut tray = match result {
        Ok(instance) => instance,

        Err(error) => {
            error::handle_error("Failed to create tray item!", error);
            error::shutdown(1);
        }
    };

    let (tx, rx) = mpsc::channel();

    enum TrayMessage {
        OpenSite,
        Die,
    }

    let open_tx = tx.clone();
    tray.add_menu_item("Open Overlay", move || {
        open_tx.send(TrayMessage::OpenSite).unwrap_or_else(|error| {
            error::handle_error(
                "Failed to send Open Overlay interaction across mpsc!",
                error,
            );
            error::shutdown(1);
        });
    })
    .unwrap_or_else(|error| {
        error::handle_error("Failed to add menu element to tray item!", error);
        error::shutdown(1);
    });

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(TrayMessage::Die).unwrap_or_else(|error| {
            error::handle_error("Failed to send Quit interaction across mpsc!", error);
            error::shutdown(1);
        });
    })
    .unwrap_or_else(|error| {
        error::handle_error("Failed to add menu element to tray item!", error);
        error::shutdown(1);
    });

    let address = format!(
        "http://127.0.0.1:{:?}",
        settings.read_config::<u16>("web_port")
    );
    loop {
        let event = match rx.recv() {
            Ok(data) => data,

            Err(_) => continue, // hopefully they press it again
        };

        match event {
            TrayMessage::OpenSite => match open::that(String::from(address.clone())) {
                Ok(_) => {}

                Err(error) => error::handle_error("Failed to open overlay in browser!", error),
            },

            TrayMessage::Die => error::shutdown(0),
        }
    }
}

fn main() {
    let settings = Settings::new("settings.json");

    let tray_settings = settings.clone();
    thread::spawn(move || {
        spawn_tray(tray_settings);
    });

    let socket_server_settings = settings.clone();
    thread::spawn(move || {
        server::spawn_socket_server(socket_server_settings);
    });

    let keyboard_settings = settings.clone();
    keyboard::hook_keyboard(keyboard_settings);

    let webserver_settings = settings.clone();
    match server::spawn_webserver(webserver_settings) {
        Ok(_) => {}

        Err(error) => {
            error::handle_error("HttpServer did not exit gracefully.", error);
            error::shutdown(1);
        }
    };

    error::shutdown(0);
}
