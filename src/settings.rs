extern crate egui_keybinds;
extern crate serde;

use crate::helper;
use anyhow::Context;
use egui_keybinds::KeyBind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct Settings {
    pub server: ServerSettings,
    pub web: WebSettings,
    pub keyboard: KeyboardSettings,
}

#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct ServerSettings {
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct WebSettings {
    pub websocket_endpoint: String,
    pub local_file_path: String,
}

#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct KeyboardSettings {
    pub keys: Vec<String>,
    pub reset: String,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 3120_u16,
        }
    }
}

impl Default for WebSettings {
    fn default() -> Self {
        Self {
            websocket_endpoint: "/ws".to_string(),
            local_file_path: "/static".to_string(),
        }
    }
}

impl Default for KeyboardSettings {
    fn default() -> Self {
        Self {
            keys: vec!["Z".to_string(), "X".to_string()],
            reset: "End".to_string(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: Default::default(),
            web: Default::default(),
            keyboard: Default::default(),
        }
    }
}

impl From<OverlaySettings> for Settings {
    fn from(mut overlay_settings: OverlaySettings) -> Self {
        let mut keys = vec![];

        for mut pair in overlay_settings.keys {
            let mut serialized = pair.0.serialize();

            if let Some(mask) = pair.1 {
                serialized.push(':');
                serialized.push_str(&mask);
            }

            keys.push(serialized);
        }

        Self {
            server: overlay_settings.server,
            web: overlay_settings.web,

            keyboard: KeyboardSettings {
                keys,
                reset: overlay_settings.reset.serialize(),
            },
        }
    }
}

// Actual settings

#[derive(Clone, Hash)]
pub struct OverlaySettings {
    pub keys: Vec<(KeyBind, Option<String>)>,
    pub reset: KeyBind,

    pub server: ServerSettings,
    pub web: WebSettings,

    pub toml_settings: Settings,
}

impl OverlaySettings {
    pub fn is_fatal_change(one: &Self, two: &Self) -> bool {
        helper::hash_of(&one.server) != helper::hash_of(&two.server)
            || helper::hash_of(&one.web) != helper::hash_of(&two.web)
    }

    pub fn to_toml(&self) -> anyhow::Result<String> {
        let new_toml_settings = Settings::from(self.clone());

        match toml::to_string_pretty(&new_toml_settings) {
            Ok(toml) => return Ok(toml),
            Err(error) => anyhow::bail!("{:?}", error),
        };
    }

    pub fn from_toml(toml_settings: Settings) -> anyhow::Result<Self> {
        let mut keys: Vec<(KeyBind, Option<String>)> = vec![];

        for str in &toml_settings.keyboard.keys {
            let data: (String, Option<String>) = if str.contains(":") {
                let split: Vec<String> = str.split(":").map(|s| s.to_string()).collect();

                let first = split
                    .first()
                    .with_context(|| "Failed to get first in split list")?
                    .to_owned();

                let last = split
                    .last()
                    .with_context(|| "Failed to get last in split list")?
                    .to_owned();

                (first, Some(last))
            } else {
                (str.to_owned(), None)
            };

            let key = KeyBind::deserialize(data.0).unwrap_or(KeyBind::empty());

            keys.push((key, data.1));
        }

        let mut reset = KeyBind::empty();

        if let Ok(key) = KeyBind::deserialize(toml_settings.keyboard.reset.clone()) {
            reset = key;
        }

        Ok(Self {
            keys,
            reset,

            server: toml_settings.server.clone(),
            web: toml_settings.web.clone(),

            toml_settings,
        })
    }
}
