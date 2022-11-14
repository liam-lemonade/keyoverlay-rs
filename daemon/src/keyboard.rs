use array_tool::vec::Intersect;

use device_query::{DeviceQuery, DeviceState, Keycode};

use std::collections::HashMap;
use std::vec::Vec;

use crate::error;
use crate::server;
use crate::settings::Settings;

fn mask_keys(pressed: Vec<String>, mask: HashMap<String, String>) -> Vec<String> {
    let mut ret = pressed;

    for e in ret.iter_mut() {
        if mask.contains_key(&e.clone()) {
            match mask.get(&e.clone()) {
                Some(found) => {
                    *e = found.to_owned();
                }

                None => {}
            }
        }
    }

    return ret;
}

pub fn hook_keyboard(settings: Settings) {
    let mut keys = settings.read_config::<Vec<String>>("keys");
    let mut mask = HashMap::<String, String>::with_capacity(keys.len());

    // create hashmap mask
    for key in settings.read_config::<Vec<String>>("keys") {
        if key.contains(":") {
            let split: Vec<&str> = key.split(":").collect();

            match split.first() {
                Some(str) => match split.last() {
                    Some(str2) => {
                        mask.insert(str.to_string(), str2.to_string()); // key:mask
                    }

                    None => {
                        error::messagebox("Failed to split key:mask at last!");
                        error::shutdown(1);
                    }
                },

                None => {
                    error::messagebox("Failed to split key:mask at first!");
                    error::shutdown(1);
                }
            }
        } else {
            mask.insert(key.clone(), key.clone()); // key:key
        }
    }

    // set keys to the actual keys to monitor
    keys = mask.keys().cloned().collect::<Vec<String>>();
    println!("Keys: {:?}\nMask: {:?}", keys, mask);

    let reset = settings.read_config::<String>("reset");

    let device = DeviceState::new();

    let mut last_pressed: Vec<String> = Vec::new();
    let mut did_reset = false;
    loop {
        let pressed_keycodes: Vec<Keycode> = device.get_keys();

        // string-ify keycode
        let mut pressed_strings: Vec<String> = Vec::new();
        for key in pressed_keycodes {
            pressed_strings.push(key.to_string());
        }

        // check what monitored keys are pressed
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
            // mask keys based on the config
            let masked_pressed = mask_keys(pressed.clone(), mask.clone());

            // send the list "pressed" to every websocket client
            let result = serde_json::to_string(&masked_pressed);

            match result {
                Ok(json) => server::update_clients(json),

                Err(error) => error::handle_error("Failed to serialize pressed keys!", error),
            }
        }

        last_pressed = pressed;
    }
}
