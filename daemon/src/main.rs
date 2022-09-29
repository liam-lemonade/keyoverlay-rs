//#![windows_subsystem = "windows"] // don't create console window

extern crate actix_web;
extern crate actix_files;
extern crate actix_web_actors;
extern crate open;

extern crate device_query;

extern crate config;
extern crate array_tool;
extern crate serde;

extern crate msgbox;
extern crate tray_item;

extern crate std;

use actix_web::{ get, App, HttpResponse, HttpServer, Responder, HttpRequest };
use actix_web_actors::ws;
use actix_files as fs;

use device_query::{ DeviceQuery, DeviceState, Keycode };

use config::Config;

use tray_item::TrayItem;

use std::io::{ Write, Read };
use std::sync::{ mpsc, Arc, Mutex };
use std::vec;
use std::thread;
use std::path::Path;
use std::fs::File;
use std::process::ExitCode;

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

static WEBFILE_PATH: &str = r"D:\code\projects\keyoverlay\web";

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

fn create_tray_item() {
    let result = TrayItem::new("KeyOverlay Daemon", "keyoverlay-icon");

    let mut tray = match result {
        Ok(instance) => instance,
        Err(_) => {
            msgbox_error("Failed to create tray item!");
            std::process::exit(1);
        }
    };

    let (tx, rx) = mpsc::channel();
    
    enum TrayMessage {
        OpenSite,
        Die
    }
    
    let open_tx = tx.clone();
    tray.add_menu_item("Open Overlay", move || {
        open_tx.send(TrayMessage::OpenSite).unwrap();
    }).unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(TrayMessage::Die).unwrap();
    }).unwrap();

    let settings = try_get_config("settings.json");
    let address = format!("http://127.0.0.1:{:?}", try_read_config::<u16>(&settings, "port"));
    loop {
        let event = rx.recv().unwrap();
        match event {
            TrayMessage::OpenSite => open::that(String::from(address.clone())).unwrap(),
            TrayMessage::Die => std::process::exit(0)
        }
    };
}

fn run_app() {
    thread::spawn(|| {
        create_tray_item();
    });

    thread::spawn(|| {
        // capture key
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_app();

    let address = ("127.0.0.1", try_read_config::<u16>(&try_get_config("settings.json"), "port"));

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new("/", WEBFILE_PATH).show_files_listing())
        })
    .bind(address)?
    .run()
    .await
}