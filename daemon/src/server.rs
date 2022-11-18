extern crate actix_files;
extern crate actix_web;
extern crate anyhow;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use actix_files as fs;
use actix_web::{middleware::TrailingSlash, App, HttpServer};
use actix_web_lab::middleware::NormalizePath;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use simple_websockets::{Event, Message, Responder};

use crate::{
    error::{self, ExitStatus},
    settings::Settings,
};

//const WEBFILE_PATH: &str = r"D:\code\projects\keyoverlay\web";

const WEBFILE_PATH: &str = if cfg!(feature = "debug") {
    r"D:\code\projects\keyoverlay\web"
} else {
    r"web"
};

lazy_static! {
    static ref CLIENT_LIST: Arc<Mutex<HashMap<u64, Responder>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

// const WEBFILE_PATH: &str = r"web"

pub fn update_clients(buffer: String) {
    thread::spawn(move || {
        //println!("Updating clients, where buffer == {}", &buffer);

        for client in CLIENT_LIST.lock().unwrap().iter() {
            client.1.send(Message::Text(buffer.clone()));
        }
    });
}

#[actix_web::main]
pub async fn spawn_webserver(settings: Settings) -> std::io::Result<()> {
    let port = settings
        .read_config::<u16>("web_port")
        .unwrap_or_else(|error| {
            error::handle_error("An error occured while running the webserver thread", error);
            error::shutdown(ExitStatus::Failure);
        });

    let address = ("127.0.0.1", port);

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

    let formatted = format!("http://{}:{}", address.0, port);
    open::that(&formatted)
        .with_context(|| format!("Failed to open {} in browser", &formatted))
        .unwrap_or_else(|error| {
            error::handle_error("An error occured while running the webserver thread", error);
            error::shutdown(ExitStatus::Failure);
        }); // actix has forced my hand with this one

    server.await
}

pub fn spawn_socket_server(settings: Settings) -> Result<()> {
    let port = settings.read_config::<u16>("socket_port")?;

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

        Err(error) => anyhow::bail!("Failed to create websocket hub: {:?}", error),
    }
}
