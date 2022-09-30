//#![windows_subsystem = "windows"] // don't create console window

extern crate tray_item;
extern crate device_query;
extern crate array_tool;

pub mod settings;
pub mod error;
pub mod server;

use settings::Settings;

use tray_item::TrayItem;

use array_tool::vec::Intersect;

use device_query::{ Keycode, DeviceState, DeviceQuery };

use std::sync::mpsc;
use std::thread;
use std::vec::Vec;

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
        Die
    }
    
    let open_tx = tx.clone();
    tray.add_menu_item("Open Overlay", move || {
        open_tx.send(TrayMessage::OpenSite).unwrap_or_else(|error| {
            error::handle_error("Failed to send Open Overlay interaction across mpsc!", error);
            error::shutdown(1);
        });
    }).unwrap_or_else(|error| {
        error::handle_error("Failed to add menu element to tray item!", error);
        error::shutdown(1);
    });

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(TrayMessage::Die).unwrap_or_else(|error| {
            error::handle_error("Failed to send Quit interaction across mpsc!", error);
            error::shutdown(1);
        });
    }).unwrap_or_else(|error| {
        error::handle_error("Failed to add menu element to tray item!", error);
        error::shutdown(1);
    });

    let address = format!("http://127.0.0.1:{:?}", settings.read_config::<u16>("port"));
    loop {
        let event = match rx.recv() {
            Ok(data) => data,

            Err(_) => continue // hopefully they press it again
        };

        match event {
            TrayMessage::OpenSite => open::that(String::from(address.clone())).unwrap(),

            TrayMessage::Die => error::shutdown(0)
        }
    };
}

fn hook_keyboard(settings: Settings) {
    let keys = settings.read_config::<Vec<String>>("keys");
    let reset = settings.read_config::<String>("reset");
    
    let device = DeviceState::new();
    let mut last_pressed: Vec<String> = Vec::new();
    loop {
        let pressed_keycodes: Vec<Keycode> = device.get_keys();

        let mut pressed_strings: Vec<String> = Vec::new();
        for key in pressed_keycodes {
            pressed_strings.push(key.to_string());
        }

        let pressed = pressed_strings.intersect(keys.clone());

        let do_reset = pressed_strings.contains(&reset);
        if pressed != last_pressed || do_reset {
            if do_reset {
                // send the message "reset" over to every websocket client
                
            }
            else {
                // send the list "pressed" to every websocket client
                
            }
        }

        last_pressed = pressed;
    }
}

fn main() {
    let settings = Settings::new("settings.json");
    
    let tray_settings = settings.clone();
    thread::spawn(move || { spawn_tray(tray_settings); });

    let keyboard_settings = settings.clone();
    thread::spawn(move || { hook_keyboard(keyboard_settings); });

    let server_settings = settings.clone();
    match server::spawn_server(server_settings) {
        Ok(_) => { },
        
        Err(error) => {
            error::handle_error("HttpServer did not exit gracefully.", error);
            error::shutdown(1);
        }
    };

    error::shutdown(0);
}