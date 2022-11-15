use device_query::DeviceEvents;
use device_query::DeviceState;

use std::collections::HashMap;
use std::vec::Vec;

use crate::error;
use crate::server;
use crate::settings::Settings;

pub fn hook_keyboard(settings: Settings) {
    let keys = settings.read_config::<Vec<String>>("keys");
    let mut key_map = HashMap::<String, String>::with_capacity(keys.len());

    // create hashmap mask
    for key in keys {
        if key.contains(":") {
            let split: Vec<&str> = key.split(":").collect();

            match split.first() {
                Some(str) => match split.last() {
                    Some(str2) => {
                        key_map.insert(str.to_string(), str2.to_string()); // key:mask
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
            key_map.insert(key.clone(), key.clone()); // key:key
        }
    }
    println!("Created key HashMap:\n{:?}\n", &key_map);

    let device_state = DeviceState::new();

    let reset = settings.read_config::<String>("reset");

    let key_map_clone = key_map.clone();
    let _guard = device_state.on_key_down(move |key| {
        let key_str = key.to_string();

        if &key_str == &reset {
            server::update_clients("reset".to_string());
            return;
        }

        if key_map_clone.contains_key(&key_str) {
            if let Some(found) = key_map_clone.get(&key_str) {
                let msg = format!("[1, \"{}\"]", found);
                server::update_clients(msg)
            }
        }
    });
    println!("Placed 'on_key_down' hook");

    let _guard = device_state.on_key_up(move |key| {
        let key_str = key.to_string();

        if key_map.contains_key(&key_str) {
            if let Some(found) = key_map.get(&key_str) {
                let msg = format!("[0, \"{}\"]", found);
                server::update_clients(msg)
            }
        }
    });
    println!("Place 'on_key_up' hook");
}
