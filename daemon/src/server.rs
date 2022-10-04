extern crate actix_files;
extern crate actix_web;
extern crate actix_web_actors;

pub mod websocket;

use actix_files as fs;
use actix_web::{
    middleware::TrailingSlash, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use actix_web_lab::middleware::NormalizePath;

use websocket::WebSocket;

use std::vec::Vec;

use crate::{error, settings::Settings};

const WEBFILE_PATH: &str = r"D:\code\projects\keyoverlay\web";
// const WEBFILE_PATH: &str = r"web"

pub async fn update_clients(pressed: Vec<String>) {
    // iterate through all websocket clients and send pressed to them
    let _ = pressed.clone();
}

pub async fn websocket_connect(
    request: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let response = ws::start(WebSocket {}, &request, stream);
    println!("{:?}", response);

    response
}

#[actix_web::main]
pub async fn spawn_server(settings: Settings) -> std::io::Result<()> {
    let address = ("127.0.0.1", settings.read_config::<u16>("port")); // the address is a tuple

    let wrapper = NormalizePath::new(TrailingSlash::Always).use_redirects();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(wrapper)
            .route("/ws", web::get().to(websocket_connect))
            .service(
                fs::Files::new("/", WEBFILE_PATH)
                    .show_files_listing()
                    .index_file("index.html"),
            )
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
