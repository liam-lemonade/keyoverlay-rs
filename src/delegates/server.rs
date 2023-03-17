extern crate anyhow;
extern crate futures;
extern crate futures_util;
extern crate lazy_static;
extern crate poem;
extern crate tokio;

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use poem::{
    endpoint::StaticFilesEndpoint,
    handler,
    listener::TcpListener,
    web::websocket::{Message, WebSocket, WebSocketStream},
    IntoResponse, Route, Server,
};

use futures_util::SinkExt;

use crate::settings::OverlaySettings;

lazy_static! {
    pub static ref CLIENT_LIST: Arc<Mutex<Vec<WebSocketStream>>> = Arc::new(Mutex::new(Vec::new()));
    //static ref CLIENT_LIST: RwLock<WebSocketStream> = RwLock::new();
}

pub fn update_clients(data: String) {
    CLIENT_LIST.lock().unwrap().retain_mut(|socket| {
        let result = futures::executor::block_on(socket.send(Message::Text(data.clone())));
        return result.is_ok();
    });
}

#[handler]
async fn websocket_connect(ws: WebSocket) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        CLIENT_LIST.lock().unwrap().push(socket);
    })
}

#[tokio::main]
pub async fn start(settings: OverlaySettings) -> anyhow::Result<()> {
    let path = settings.web.local_file_path;
    let address = format!("{}:{}", settings.server.ip, settings.server.port);

    // create local file endpoint hosted on /
    let file_endpoint = StaticFilesEndpoint::new(path)
        .show_files_listing()
        .redirect_to_slash_directory()
        .index_file("index.html");

    let app = Route::new().nest("/", file_endpoint).at(
        settings.web.websocket_endpoint,
        poem::get(websocket_connect),
    );

    if let Err(error) = Server::new(TcpListener::bind(address)).run(app).await {
        anyhow::bail!("{:?}", error);
    }

    Ok(())
}
