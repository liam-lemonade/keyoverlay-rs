extern crate actix_files;
extern crate actix_web;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_files as fs;
use actix_web::{middleware::TrailingSlash, App, HttpServer};
use actix_web_lab::middleware::NormalizePath;
use lazy_static::lazy_static;
use simple_websockets::{Event, Message, Responder};

use crate::{error, settings::Settings};

//const WEBFILE_PATH: &str = r"D:\code\projects\keyoverlay\web";
const WEBFILE_PATH: &str = r"web";

lazy_static! {
    static ref CLIENT_LIST: Arc<Mutex<HashMap<u64, Responder>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

// const WEBFILE_PATH: &str = r"web"

pub fn update_clients(buffer: String) {
    for client in CLIENT_LIST.lock().unwrap().iter() {
        client.1.send(Message::Text(buffer.clone()));
    }
}

#[actix_web::main]
pub async fn spawn_webserver(settings: Settings) -> std::io::Result<()> {
    let address = ("127.0.0.1", settings.read_config::<u16>("web_port")); // the address is a tuple

    let wrapper = NormalizePath::new(TrailingSlash::Always).use_redirects();

    let server = HttpServer::new(move || {
        App::new()
            .service(
                fs::Files::new("/", WEBFILE_PATH)
                    .show_files_listing()
                    .index_file("index.html"),
            )
            .wrap(wrapper)
    })
    .bind(address)?
    .run();

    let (ip, port) = address;
    match open::that(format!("http://{}:{}", ip, port)) {
        Ok(_) => {}

        Err(error) => error::handle_error("Failed to open overlay in browser!", error),
    }

    server.await
}

pub fn spawn_socket_server(settings: Settings) {
    let port = settings.read_config::<u16>("socket_port");

    match simple_websockets::launch(port) {
        Ok(socket) => {
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

                let mut list = CLIENT_LIST.lock().unwrap();
                *list = clients.clone();
            }
        }

        Err(error) => error::handle_error("Failed to create websocket server!", error),
    }
}
