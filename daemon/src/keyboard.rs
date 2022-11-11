use array_tool::vec::Intersect;

use device_query::{DeviceQuery, DeviceState, Keycode};

use std::vec::Vec;

use crate::error;
use crate::server;
use crate::settings::Settings;

pub fn hook_keyboard(settings: Settings) {
    let keys = settings.read_config::<Vec<String>>("keys");
    let reset = settings.read_config::<String>("reset");

    let device = DeviceState::new();

    let mut last_pressed: Vec<String> = Vec::new();
    let mut did_reset = false;
    loop {
        let pressed_keycodes: Vec<Keycode> = device.get_keys();

        let mut pressed_strings: Vec<String> = Vec::new();
        for key in pressed_keycodes {
            pressed_strings.push(key.to_string());
        }

        let pressed = pressed_strings.intersect(keys.clone());

        if pressed_strings.contains(&reset) {
            if !did_reset {
                // send "reset" to every websocket client
                server::update_clients("reset".to_string());

                did_reset = true;
            }
        } else {
            did_reset = false;
        }

        if pressed != last_pressed {
            // send the list "pressed" to every websocket client
            let result = serde_json::to_string(&pressed);

            match result {
                Ok(json) => server::update_clients(json),

                Err(error) => error::handle_error("Failed to serialize pressed keys!", error),
            }
        }

        last_pressed = pressed;
    }
}
