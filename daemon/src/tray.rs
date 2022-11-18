extern crate anyhow;
extern crate tray_item;

#[cfg(target_os = "windows")] // EDIT THIS LATER
extern crate libappindicator;

use std::sync::mpsc::{self, Sender};

use anyhow::{Context, Result};
use tray_item::TrayItem;

#[cfg(target_os = "windows")] // EDIT THIS LATER
use libappindicator::gtk;

use crate::{
    error::{self, ExitStatus},
    settings::Settings,
};

#[derive(Debug, Clone)]
enum TrayMessage {
    Open,
    Quit,
}

fn send_traymessage(sender: &Sender<TrayMessage>, msg: TrayMessage) {
    if let Err(error) = sender.send(msg) {
        // tray-item has forced my hand. i cant use a future, option, result, etc
        // on the bright side, this error is unrecoverable anyway
        error::handle_error(
            "An error occured while sending TrayMessage across mpsc::channel",
            error,
        );

        error::shutdown(ExitStatus::Failure);
    }
}

fn create_tray() -> Result<TrayItem> {
    let mut tray =
        TrayItem::new("keyoverlay-rs", "").with_context(|| "Failed to create tray item")?;

    match std::env::consts::OS {
        "windows" => {
            tray.set_icon("keyoverlay-icon-windows")?;
        }

        "linux" => {
            // do something
            // do smth else
            gtk::init()?;
            tray.set_icon("keyoverlay-icon-linux")?;
        }

        "macos" => {
            // do smth
            tray.set_icon("keyoverlay-icon-macos")?;
        }

        _ => anyhow::bail!("Failed to set icon, unknown operating-system"),
    };

    Ok(tray)
}

pub fn handle_tray(settings: Settings) -> Result<()> {
    let mut tray = create_tray()?;

    let (tx, rx) = mpsc::channel();

    let open_tx = tx.clone();

    tray.add_menu_item("Open", move || {
        send_traymessage(&open_tx, TrayMessage::Open);
    })?;

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        send_traymessage(&quit_tx, TrayMessage::Quit);
    })?;

    let address = format!(
        "http://127.0.0.1:{:?}",
        settings.read_config::<u16>("web_port")
    );
    loop {
        let event = rx.recv()?; // big issue if this doesnt recieve, the channel has closed

        match event {
            TrayMessage::Open => {
                open::that(String::from(address.clone()))?;
            }

            TrayMessage::Quit => error::shutdown(ExitStatus::Success),
        }
    }
}
