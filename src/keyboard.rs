extern crate lazy_static;
extern crate rdev;

use std::{collections::HashMap, sync::RwLock};

use crate::{server, SETTINGS};
use anyhow::Context;
use lazy_static::lazy_static;
use rdev::{Event, EventType, Key};

lazy_static! {
    static ref KEY_MAP: RwLock<HashMap<String, (String, usize)>> = RwLock::new(HashMap::new());
    static ref RESET: RwLock<String> = RwLock::new(String::new());
    static ref PRESSED_KEYS: RwLock<Vec<String>> = RwLock::new(Vec::new());
}

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

pub fn build_keymap() -> anyhow::Result<()> {
    let settings;
    {
        settings = SETTINGS.read().unwrap().clone()
    }

    let unmasked_list: Vec<String> = settings.keyboard.keys;

    let mut masked_keys: HashMap<String, (String, usize)> = HashMap::new();
    for (i, key) in unmasked_list.iter().enumerate() {
        if key.contains(":") {
            let split: Vec<String> = key.split(":").map(|s| s.to_string()).collect();

            let first = split
                .first()
                .with_context(|| "Failed to get first element of split")?;

            let second = split
                .last()
                .with_context(|| "Failed to get last element of split")?;

            masked_keys.insert((*first).to_lowercase(), ((*second).clone(), i));
        } else {
            masked_keys.insert(key.to_lowercase(), (key.clone(), i));
        }
    }

    *KEY_MAP.write().unwrap() = masked_keys;
    *RESET.write().unwrap() = settings.keyboard.reset;

    Ok(())
}

fn handle_key(k: Key, down: bool) {
    // this stuff is a little performance intensive (not really but its still a waste) so we don't want to run it unless its a keyboard message
    // rdev also does stuff like mouse
    let mut pressed_keys = { PRESSED_KEYS.read().unwrap().clone() };
    let reset = { RESET.read().unwrap().clone() };
    let key_map = { KEY_MAP.read().unwrap().clone() };

    let key = key_to_string(k).to_lowercase();

    if down {
        if pressed_keys.contains(&key) {
            return;
        }
    } else {
        if key == reset.to_lowercase() {
            server::update_clients("reset".to_string());
            return;
        }
    }

    if let Some(pair) = key_map.get(&key) {
        let (mask, index) = pair;

        server::update_clients(format!("[\"{}\", {}, {}]", mask, down, index));

        if down {
            pressed_keys.push(key);
        } else {
            if let Some(index) = pressed_keys.iter().position(|x| *x == key) {
                pressed_keys.remove(index);
            }
        }
    }

    *PRESSED_KEYS.write().unwrap() = pressed_keys;
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(k) => {
            handle_key(k, true);
        }

        EventType::KeyRelease(k) => {
            handle_key(k, false);
        }

        _ => (),
    }
}

pub fn start() -> anyhow::Result<()> {
    build_keymap()?;

    if let Err(err) = rdev::listen(callback) {
        anyhow::bail!("{:?}", err);
    }

    Ok(())
}
