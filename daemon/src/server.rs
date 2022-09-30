extern crate actix_files;
extern crate actix_web;
extern crate actix_web_actors;

use actix_files as fs;
use actix_web::{middleware::TrailingSlash, App, HttpServer};
use actix_web_actors::ws;
use actix_web_lab::middleware::NormalizePath;

use std::vec::Vec;

use crate::{error, settings::Settings};

const WEBFILE_PATH: &str = r"D:\code\projects\keyoverlay\web";
// const WEBFILE_PATH: &str = r"web"

pub async fn update_clients(_pressed: Vec<String>) {
    // iterate through all websocket clients and send pressed to them
}

#[actix_web::main]
pub async fn spawn_server(settings: Settings) -> std::io::Result<()> {
    let address = ("127.0.0.1", settings.read_config::<u16>("port")); // the address is a tuple

    let wrapper = NormalizePath::new(TrailingSlash::Always).use_redirects();

    let server = HttpServer::new(move || {
        App::new().wrap(wrapper).service(
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
