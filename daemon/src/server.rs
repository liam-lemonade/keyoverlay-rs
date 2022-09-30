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

#[get("/{dir}")]
async fn index(request: HttpRequest) -> impl Responder {
    let dir: String = request.match_info().query("dir").parse().unwrap();
    let path = format!("{}\\{}\\index.html", WEBFILE_PATH, dir);

    let result = fs::NamedFile::open(path);

    match result {
        Ok(mut file) => {
            let mut html: String = String::new();
            _ = file.read_to_string(&mut html);

            return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html);
        }

        Err(error) => {
            return HttpResponse::Ok().body(error.to_string());
        }
    } 
}

#[actix_web::main]
pub async fn spawn_server(settings: Settings) ->  std::io::Result<()> {
    let address = ("127.0.0.1", settings.read_config::<u16>("port")); // the address is a tuple

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new("/", WEBFILE_PATH).show_files_listing())
        })
    .bind(address)?
    .run()
    .await
}