extern crate anyhow;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::vec::Vec;

use crate::server;
use crate::settings::Settings;
use rdev::{Event, EventType, Key};

pub fn key_as_str(key: Key) -> &'static str {
    match key {
        Key::Alt => "Alt",
        Key::AltGr => "AltGr",
        Key::Backspace => "Backspace",
        Key::CapsLock => "CapsLock",
        Key::ControlLeft => "ControlLeft",
        Key::ControlRight => "ControlRight",
        Key::Delete => "Delete",
        Key::DownArrow => "ArrowDown",
        Key::End => "End",
        Key::Escape => "Escape",
        Key::F1 => "F1",
        Key::F10 => "F10",
        Key::F11 => "F11",
        Key::F12 => "F12",
        Key::F2 => "F2",
        Key::F3 => "F3",
        Key::F4 => "F4",
        Key::F5 => "F5",
        Key::F6 => "F6",
        Key::F7 => "F7",
        Key::F8 => "F8",
        Key::F9 => "F9",
        Key::Home => "Home",
        Key::LeftArrow => "ArrowLeft",
        Key::MetaLeft => "MetaLeft",
        Key::MetaRight => "MetaRight",
        Key::PageDown => "PageDown",
        Key::PageUp => "PageUp",
        Key::Return => "Enter",
        Key::RightArrow => "ArrowRight",
        Key::ShiftLeft => "ShiftLeft",
        Key::ShiftRight => "ShiftRight",
        Key::Space => "Space",
        Key::Tab => "Tab",
        Key::UpArrow => "ArrowUp",
        Key::PrintScreen => "PrintScreen",
        Key::ScrollLock => "ScrollLock",
        Key::Pause => "Pause",
        Key::NumLock => "Pause",
        Key::BackQuote => "`",
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "4",
        Key::Num5 => "5",
        Key::Num6 => "6",
        Key::Num7 => "7",
        Key::Num8 => "8",
        Key::Num9 => "9",
        Key::Num0 => "0",
        Key::Minus => "-",
        Key::Equal => "=",
        Key::KeyQ => "Q",
        Key::KeyW => "W",
        Key::KeyE => "E",
        Key::KeyR => "R",
        Key::KeyT => "T",
        Key::KeyY => "Y",
        Key::KeyU => "U",
        Key::KeyI => "I",
        Key::KeyO => "O",
        Key::KeyP => "P",
        Key::LeftBracket => "[",
        Key::RightBracket => "]",
        Key::KeyA => "A",
        Key::KeyS => "S",
        Key::KeyD => "D",
        Key::KeyF => "F",
        Key::KeyG => "G",
        Key::KeyH => "H",
        Key::KeyJ => "J",
        Key::KeyK => "K",
        Key::KeyL => "L",
        Key::SemiColon => ";",
        Key::Quote => "Quote",
        Key::BackSlash => "\\",
        Key::IntlBackslash => "\\",
        Key::KeyZ => "Z",
        Key::KeyX => "X",
        Key::KeyC => "C",
        Key::KeyV => "V",
        Key::KeyB => "B",
        Key::KeyN => "N",
        Key::KeyM => "M",
        Key::Comma => ",",
        Key::Dot => ".",
        Key::Slash => "/",
        Key::Insert => "Insert",
        Key::KpReturn => "Return",
        Key::KpMinus => "-",
        Key::KpPlus => "+",
        Key::KpMultiply => "*",
        Key::KpDivide => "/",
        Key::Kp0 => "0",
        Key::Kp1 => "1",
        Key::Kp2 => "2",
        Key::Kp3 => "3",
        Key::Kp4 => "4",
        Key::Kp5 => "5",
        Key::Kp6 => "6",
        Key::Kp7 => "7",
        Key::Kp8 => "8",
        Key::Kp9 => "9",
        Key::KpDelete => "Delete",
        Key::Function => "Function",
        Key::Unknown(_) => "Unknown",
    }
}

pub fn key_to_string(key: Key) -> String {
    return key_as_str(key).to_string();
}

pub fn build_keymap(keys: Vec<String>) -> Result<HashMap<String, (String, usize)>> {
    let mut key_map = HashMap::<String, (String, usize)>::with_capacity(keys.len());

    for (i, key) in keys.iter().enumerate() {
        if !key.contains(":") {
            key_map.insert(key.clone().to_lowercase(), (key.to_owned(), i));
            continue;
        }

        let split: Vec<String> = key.split(":").map(|s| s.to_string()).collect();

        let first = split
            .first()
            .with_context(|| "Failed to get split.first()")?;

        let last = split.last().with_context(|| "Failed to get split.last()")?;

        key_map.insert(first.to_owned().to_lowercase(), (last.to_owned(), i));
    }

    return Ok(key_map);
}

pub fn hook_keyboard(settings: Settings) -> Result<()> {
    let keys = build_keymap(settings.read_config::<Vec<String>>("keys")?)?;

    let reset = settings.read_config::<String>("reset")?.to_lowercase();

    let mut held_keys: Vec<String> = Vec::new();

    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyRelease(key) => {
                let key_str = key_to_string(key).to_lowercase();

                if key_str == reset {
                    server::update_clients("reset".to_string());
                    return;
                }

                if let Some(pair) = keys.get(&key_str) {
                    let (mask, index) = pair;

                    // key_str is the monitored key, mask is the value to send
                    // send it to the clients
                    server::update_clients(format!("[\"{}\", false, {}]", mask, index));

                    if let Some(index) = held_keys.iter().position(|x| *x == key_str) {
                        held_keys.remove(index);
                    }
                }
            }

            EventType::KeyPress(key) => {
                let key_str = key_to_string(key).to_lowercase();

                if held_keys.contains(&key_str) {
                    return;
                }

                if let Some(pair) = keys.get(&key_str) {
                    let (mask, index) = pair;

                    // key_str is the monitored key, mask is the value to send
                    // send it to the clients
                    server::update_clients(format!("[\"{}\", true, {}]", mask, index));
                    held_keys.push(key_str);
                }
            }

            _ => {}
        }
    };

    if let Err(error) = rdev::listen(callback) {
        anyhow::bail!("Error while listening to keyboard input: {:?}", error);
    }

    Ok(())
}
