extern crate actix_web;
extern crate actix_files;
extern crate actix_web_actors;

use actix_web::{ get, App, HttpResponse, HttpServer, Responder, HttpRequest };
use actix_web_actors::ws;
use actix_files as fs;

use std::io::Read;
use std::vec::Vec;

use crate::settings::Settings;

const WEBFILE_PATH: &str = r"D:\code\projects\keyoverlay\web";
// const WEBFILE_PATH: &str = r"web"

pub async fn update_clients(pressed: Vec<String>) {
    // iterate through all websocket clients and send pressed to them
}

#[actix_web::main]
pub async fn spawn_server(settings: Settings) ->  std::io::Result<()> {
    let address = ("127.0.0.1", settings.read_config::<u16>("port")); // the address is a tuple

    let server = HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", WEBFILE_PATH)
                .show_files_listing()
                .index_file("index.html")
            )
        })
    .bind(address)?
    .run();
    
    let (ip, port) = address;
    open::that(format!("http://{}:{}", ip, port)).unwrap();

    server.await
}