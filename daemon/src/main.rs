use device_query::{DeviceQuery, DeviceState, Keycode};
use simple_websockets::{Event, Responder};
use config::Config;

use std::io::Write;
use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use std::thread;
use std::path::Path;
use std::fs::File;

use array_tool::vec::Intersect;

static DEFAULT_CONFIG: &[u8] = 
b"{
    \"port\": 7685,
    \"keys\": [ \"Z\", \"X\" ]
}";

fn create_default_config(name: &str) {
    let mut file = File::create(name).unwrap();
    file.write_all(DEFAULT_CONFIG).unwrap();
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

fn main() {
    // variables
    let settings = try_get_config("settings.json"); 
    let client_list = Arc::new(Mutex::new(HashMap::new()));

    let port = settings.get::<u16>("port").unwrap();
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

    let key_clone = Arc::clone(&client_list);
    
    let device_state = DeviceState::new();
    let mut last_pressed: Vec<String> = Vec::new();
    loop {
        let pressed: Vec<Keycode> = device_state.get_keys();

        let mut strings: Vec<String> = Vec::new();
        for key in pressed.iter() {
             strings.push(key.to_string()) 
        }

        let intersect = strings.intersect(settings.get::<Vec<String>>("keys").unwrap());

        if intersect != last_pressed {

            let clients = key_clone.lock().unwrap();
            for (_, client) in clients.clone() {
                client.send(simple_websockets::Message::Text(format!("{:?}", intersect)));
            }

            last_pressed = intersect;
        }
    }
}