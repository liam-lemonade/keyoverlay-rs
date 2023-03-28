use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_of<T: Hash>(t: T) -> u64 {
    let mut state = DefaultHasher::new();
    t.hash(&mut state);

    state.finish()
}

use std::{fs::File, io::Write};

use anyhow::Context;
use egui_keybinds::KeyCode;

use crate::settings::Settings;

pub fn is_first_run(path: &str) -> bool {
    !std::path::Path::new(path).exists()
}

pub fn create_default_file(path: &str, toml_settings: Settings) -> anyhow::Result<()> {
    let mut file =
        File::create(path).with_context(|| "Failed to create default configuration file")?;

    let data = toml::to_string_pretty(&toml_settings)
        .with_context(|| "Failed to serialize default settings")?;

    file.write(data.as_bytes())
        .with_context(|| "Failed to write to default configuration file")?;

    Ok(())
}

pub fn rdev_to_egui(key: rdev::Key) -> anyhow::Result<KeyCode> {
    match key {
        rdev::Key::Alt => Ok(KeyCode::LAlt),
        rdev::Key::AltGr => Ok(KeyCode::LAlt),
        rdev::Key::Backspace => Ok(KeyCode::Backspace),
        rdev::Key::CapsLock => Ok(KeyCode::CapsLock),
        rdev::Key::ControlLeft => Ok(KeyCode::LControl),
        rdev::Key::ControlRight => Ok(KeyCode::RControl),
        rdev::Key::Delete => Ok(KeyCode::Delete),
        rdev::Key::DownArrow => Ok(KeyCode::DownArrow),
        rdev::Key::End => Ok(KeyCode::End),
        rdev::Key::Escape => Ok(KeyCode::Escape),
        rdev::Key::F1 => Ok(KeyCode::F1),
        rdev::Key::F10 => Ok(KeyCode::F10),
        rdev::Key::F11 => Ok(KeyCode::F11),
        rdev::Key::F12 => Ok(KeyCode::F12),
        rdev::Key::F2 => Ok(KeyCode::F2),
        rdev::Key::F3 => Ok(KeyCode::F3),
        rdev::Key::F4 => Ok(KeyCode::F4),
        rdev::Key::F5 => Ok(KeyCode::F5),
        rdev::Key::F6 => Ok(KeyCode::F6),
        rdev::Key::F7 => Ok(KeyCode::F7),
        rdev::Key::F8 => Ok(KeyCode::F8),
        rdev::Key::F9 => Ok(KeyCode::F9),
        rdev::Key::Home => Ok(KeyCode::Home),
        rdev::Key::LeftArrow => Ok(KeyCode::LeftArrow),
        rdev::Key::MetaLeft => Ok(KeyCode::LWindows),
        rdev::Key::MetaRight => Ok(KeyCode::RWindows),
        rdev::Key::PageDown => Ok(KeyCode::PageDown),
        rdev::Key::PageUp => Ok(KeyCode::PageUp),
        rdev::Key::Return => Ok(KeyCode::Return),
        rdev::Key::RightArrow => Ok(KeyCode::RightArrow),
        rdev::Key::ShiftLeft => Ok(KeyCode::LShift),
        rdev::Key::ShiftRight => Ok(KeyCode::RShift),
        rdev::Key::Space => Ok(KeyCode::Space),
        rdev::Key::Tab => Ok(KeyCode::Tab),
        rdev::Key::UpArrow => Ok(KeyCode::UpArrow),
        rdev::Key::PrintScreen => anyhow::bail!("PrintScreen not supported yet!"),
        rdev::Key::ScrollLock => anyhow::bail!("ScrollLock not supported yet!"),
        rdev::Key::Pause => anyhow::bail!("Pause not supported yet!"),
        rdev::Key::NumLock => anyhow::bail!("NumLock not supported yet!"),
        rdev::Key::BackQuote => Ok(KeyCode::Backtick),
        rdev::Key::Num1 => Ok(KeyCode::Num1),
        rdev::Key::Num2 => Ok(KeyCode::Num2),
        rdev::Key::Num3 => Ok(KeyCode::Num3),
        rdev::Key::Num4 => Ok(KeyCode::Num4),
        rdev::Key::Num5 => Ok(KeyCode::Num5),
        rdev::Key::Num6 => Ok(KeyCode::Num6),
        rdev::Key::Num7 => Ok(KeyCode::Num7),
        rdev::Key::Num8 => Ok(KeyCode::Num8),
        rdev::Key::Num9 => Ok(KeyCode::Num9),
        rdev::Key::Num0 => Ok(KeyCode::Num0),
        rdev::Key::Minus => Ok(KeyCode::Minus),
        rdev::Key::Equal => Ok(KeyCode::Equals),
        rdev::Key::KeyQ => Ok(KeyCode::Q),
        rdev::Key::KeyW => Ok(KeyCode::W),
        rdev::Key::KeyE => Ok(KeyCode::E),
        rdev::Key::KeyR => Ok(KeyCode::R),
        rdev::Key::KeyT => Ok(KeyCode::T),
        rdev::Key::KeyY => Ok(KeyCode::Y),
        rdev::Key::KeyU => Ok(KeyCode::U),
        rdev::Key::KeyI => Ok(KeyCode::I),
        rdev::Key::KeyO => Ok(KeyCode::O),
        rdev::Key::KeyP => Ok(KeyCode::P),
        rdev::Key::LeftBracket => Ok(KeyCode::OpenBracket),
        rdev::Key::RightBracket => Ok(KeyCode::CloseBracket),
        rdev::Key::KeyA => Ok(KeyCode::A),
        rdev::Key::KeyS => Ok(KeyCode::S),
        rdev::Key::KeyD => Ok(KeyCode::D),
        rdev::Key::KeyF => Ok(KeyCode::F),
        rdev::Key::KeyG => Ok(KeyCode::G),
        rdev::Key::KeyH => Ok(KeyCode::H),
        rdev::Key::KeyJ => Ok(KeyCode::J),
        rdev::Key::KeyK => Ok(KeyCode::K),
        rdev::Key::KeyL => Ok(KeyCode::L),
        rdev::Key::SemiColon => Ok(KeyCode::SemiColon),
        rdev::Key::Quote => Ok(KeyCode::Apostrophe),
        rdev::Key::BackSlash => Ok(KeyCode::Backslash),
        rdev::Key::IntlBackslash => Ok(KeyCode::Backslash),
        rdev::Key::KeyZ => Ok(KeyCode::Z),
        rdev::Key::KeyX => Ok(KeyCode::X),
        rdev::Key::KeyC => Ok(KeyCode::C),
        rdev::Key::KeyV => Ok(KeyCode::V),
        rdev::Key::KeyB => Ok(KeyCode::B),
        rdev::Key::KeyN => Ok(KeyCode::N),
        rdev::Key::KeyM => Ok(KeyCode::M),
        rdev::Key::Comma => Ok(KeyCode::Comma),
        rdev::Key::Dot => Ok(KeyCode::Period),
        rdev::Key::Slash => Ok(KeyCode::ForwardSlash),
        rdev::Key::Insert => Ok(KeyCode::Insert),
        rdev::Key::KpReturn => Ok(KeyCode::Return),
        rdev::Key::KpMinus => Ok(KeyCode::Minus),
        rdev::Key::KpPlus => Ok(KeyCode::Plus),
        rdev::Key::KpMultiply => Ok(KeyCode::Asterisks),
        rdev::Key::KpDivide => Ok(KeyCode::ForwardSlash),
        rdev::Key::Kp0 => Ok(KeyCode::Num0),
        rdev::Key::Kp1 => Ok(KeyCode::Num1),
        rdev::Key::Kp2 => Ok(KeyCode::Num2),
        rdev::Key::Kp3 => Ok(KeyCode::Num3),
        rdev::Key::Kp4 => Ok(KeyCode::Num4),
        rdev::Key::Kp5 => Ok(KeyCode::Num5),
        rdev::Key::Kp6 => Ok(KeyCode::Num6),
        rdev::Key::Kp7 => Ok(KeyCode::Num7),
        rdev::Key::Kp8 => Ok(KeyCode::Num8),
        rdev::Key::Kp9 => Ok(KeyCode::Num9),
        rdev::Key::KpDelete => Ok(KeyCode::Delete),
        rdev::Key::Function => Ok(KeyCode::LFunction),
        rdev::Key::Unknown(code) => anyhow::bail!("Unknown rdev key: {}", code),
    }
}
