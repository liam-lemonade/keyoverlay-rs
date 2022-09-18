use device_query::{DeviceQuery, DeviceState, Keycode};
use simple_websockets::{Event, Responder};
use config::Config;

use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use array_tool::vec::Intersect;
use std::thread;

fn main() {
    let settings = 
        Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()
        .unwrap();
    
    let client_list: Arc<Mutex<HashMap<u64, Responder>>> = Arc::new(Mutex::new(HashMap::new()));

    let port = settings.get::<u16>("port").unwrap();
    let web_clone = Arc::clone(&client_list);
    thread::spawn(move || {
        let socket = simple_websockets::launch(port)
            .expect(format!("Failed to create websocket on port {:?}", port).as_str());

        // ulong id : responder struct
        let mut clients: HashMap<u64, Responder> = HashMap::new();

        loop {
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