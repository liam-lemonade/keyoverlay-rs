#![windows_subsystem = "windows"] // don't create console window

extern crate device_query;
extern crate simple_websockets;
extern crate std;
extern crate config;
extern crate msgbox;
extern crate array_tool;
extern crate serde;
extern crate tray_item;

use device_query::{DeviceQuery, DeviceState, Keycode};

use simple_websockets::{Event, Responder, Message};

use config::Config;

use tray_item::TrayItem;

use std::io::Write;
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use std::path::Path;
use std::fs::File;

use array_tool::vec::Intersect;

static DEFAULT_CONFIG: &[u8] = 
b"{
    \"port\": 7685,
    \"keys\": [ \"Z\", \"X\" ],
    \"reset\": \"End\"
}";

static MSGBOX_NEW_TEXT: &str = 
r"This seems to be the first time you've opened this application.
A default configuration (settings.json) will be created.

If you find any issues, report them on the issues tab on the GitHub.";

static MSGBOX_CONFIG_ERROR: &str =
"An error has been encountered! Attempted to read {:?} from configuration file (could be corrupt?).\n
Deleting the file (settings.json) and relaunching the program may fix this issue";

fn create_default_config(name: &str) {
    msgbox::create("KeyOverlay Daemon", MSGBOX_NEW_TEXT, msgbox::IconType::Info).unwrap();

    let mut file = File::create(name).unwrap();
    file.write_all(DEFAULT_CONFIG).unwrap();
}

fn msgbox_error(text: &str) {
    msgbox::create("KeyOverlay Daemon", text, msgbox::IconType::Error).unwrap();
}

fn try_get_config(name: &str) -> Config {
    if !Path::new(name).exists() {
        create_default_config(name);
    }

    let result =
        Config::builder()
        .add_source(config::File::with_name(name))
        .build();
    
    return match result {
        Ok(file) => file,

        other_error => panic!("Failed to open configuration file. {:?}", other_error)
    };
}

fn try_read_config<'de, T: serde::Deserialize<'de>>(config: &Config, key: &str) -> T {
    let result = config.get::<T>(key);

    // check the success of our read
    let value: T = match result {
        Ok(val) => val,
        Err(_) => {
            msgbox_error(MSGBOX_CONFIG_ERROR);
            std::process::exit(0);
        }
    };

    return value;
}

fn main() {
    // variables
    let settings = try_get_config("settings.json"); 

    // mutex to pass between threads
    let client_list = Arc::new(Mutex::new(HashMap::new()));

    let port = try_read_config::<u16>(&settings, "port");

    // mutex for the web thread
    let web_clone = Arc::clone(&client_list);
    thread::spawn(move || {
        let socket = simple_websockets::launch(port)
            .expect(format!("Failed to create websocket on port {:?}", port).as_str());

        // ulong id : responder struct
        let mut clients: HashMap<u64, Responder> = HashMap::new();

        loop {
            // socket.poll_event is a blocking function, so we made a new thread for it.
            // when something happens, it will exit the match statement, update `list` and then resume waiting
            match socket.poll_event() {
                Event::Connect(client_id, responder) => {
                    println!("Client #{} connected.", client_id);
                    clients.insert(client_id, responder);
                }

                Event::Disconnect(client_id) => {
                    println!("Client #{} disconnected.", client_id);
                    clients.remove(&client_id);
                }

                _ => (),
            }

            let mut list = web_clone.lock().unwrap();
            *list = clients.clone();
        }
    });

    // mutex for the keyboard thread
    let key_clone = Arc::clone(&client_list);
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut last_pressed: Vec<String> = Vec::new();
        loop {
            let pressed: Vec<Keycode> = device_state.get_keys();

            let mut strings: Vec<String> = Vec::new();
            for key in pressed.iter() {
                strings.push(key.to_string()) 
            }

            let do_reset = strings.contains(&try_read_config::<String>(&settings, "reset"));
            let intersect = strings.intersect(try_read_config::<Vec<String>>(&settings, "keys"));

            if intersect != last_pressed || do_reset {
                let clients = key_clone.lock().unwrap();
                for (_, client) in clients.clone() {
                    if do_reset { // index.js handles the resetting
                        client.send(Message::Text(String::from("reset")));
                    }
                    else {
                        client.send(Message::Text(format!("{:?}", intersect)));
                    }
                }

                last_pressed = intersect;
            }
        }
    });

    let result = TrayItem::new("KeyOverlay Daemon", "keyoverlay-icon");

    let mut tray = match result {
        Ok(instance) => instance,
        Err(error) => panic!("{:?}", error)
    };

    let (tx, rx) = mpsc::channel();

    tray.add_menu_item("Quit", move || {
        tx.send(String::new()).unwrap();
    }).unwrap();

    loop {
        match rx.recv() {
            Ok(_) => break,
            _ => {}
        }
    }
}