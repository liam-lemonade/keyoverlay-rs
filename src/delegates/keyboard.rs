extern crate egui_keybinds;
extern crate lazy_static;
extern crate rdev;

use anyhow::Context;
use egui_keybinds::{KeyBind, KeyCode};
use lazy_static::lazy_static;
use rdev::{Event, EventType};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::{self, Sender};
use std::sync::RwLock;
use std::thread;

use crate::{helper, settings::OverlaySettings};

use super::server;

lazy_static! {
    static ref KEYS: RwLock<Vec<(KeyBind, Option<String>)>> = RwLock::new(Vec::new());
    static ref RESET: RwLock<KeyBind> = RwLock::new(KeyBind::empty());
    static ref HELD_KEYS: RwLock<Vec<KeyCode>> = RwLock::new(Vec::new());
}

pub fn refresh_keys(keys: Vec<(KeyBind, Option<String>)>, reset: KeyBind) -> anyhow::Result<()> {
    match KEYS.write() {
        Ok(mut lock) => *lock = keys,
        Err(err) => anyhow::bail!("{:?}", err),
    }

    match RESET.write() {
        Ok(mut lock) => *lock = reset,
        Err(err) => anyhow::bail!("{:?}", err),
    }

    Ok(())
}

fn on_key_interact(rdev_key: rdev::Key, is_down: bool) -> anyhow::Result<()> {
    let keys = KEYS.read().unwrap().clone();
    let reset = RESET.read().unwrap().clone();

    let keycode = helper::rdev_to_egui(rdev_key).with_context(|| "Failed to parse rdev key")?;

    if reset.key.is_some() && reset.key.unwrap() == keycode.clone() {
        if !is_down {
            // sent "reset" to clients
            server::update_clients("reset".to_string());
        }

        return Ok(());
    }

    for (i, pair) in keys.iter().enumerate() {
        let (bind, mask) = pair.clone();

        if bind.key.is_none() {
            continue;
        }

        let bind_key = bind.key.unwrap().clone();

        if bind_key != keycode.clone() {
            continue;
        }

        let held_keys_reader = HELD_KEYS.read().unwrap().clone();
        let mut held_keys_writer = HELD_KEYS.write().unwrap();

        if is_down {
            if held_keys_reader.contains(&bind_key) {
                continue;
            }

            (*held_keys_writer).push(keycode.clone());
        } else {
            if !held_keys_reader.contains(&bind_key) {
                continue;
            }

            (*held_keys_writer).retain(|k| *k != keycode);
        }

        let displayed_key = if let Some(str) = mask {
            str
        } else {
            bind_key.serialize()
        };

        let data = format!("[\"{}\", {}, {}]", displayed_key, is_down, i);
        server::update_clients(data);
    }

    Ok(())
}

pub fn start(settings: OverlaySettings) -> anyhow::Result<()> {
    self::refresh_keys(settings.keys, settings.reset)?;

    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
    thread::spawn(move || {
        let sender_clone = sender.clone();

        let closure = move |event: Event| {
            if let Err(err) = || -> anyhow::Result<()> {
                match event.event_type {
                    EventType::KeyPress(rdev_key) => {
                        on_key_interact(rdev_key, true)?;
                    }

                    EventType::KeyRelease(rdev_key) => {
                        on_key_interact(rdev_key, false)?;
                    }

                    _ => (),
                }

                Ok(())
            }() {
                sender_clone.send(format!("{:?}", err)).unwrap();
            }
        };

        if let Err(err) = rdev::listen(closure) {
            sender.send(format!("{:?}", err)).unwrap();
        }
    });

    match receiver.recv() {
        Ok(message) => {
            anyhow::bail!(message);
        }

        Err(_) => Ok(()), // channel hung up, either we exit gracefully or it crashed and burned
    }
}
