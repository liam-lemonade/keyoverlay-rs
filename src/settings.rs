extern crate egui_keybinds;
extern crate serde;

use crate::helper;
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
    pub port: i16,
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
            port: 3120_i16,
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

        for mut key in overlay_settings.keys {
            keys.push(key.serialize());
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
    pub keys: Vec<KeyBind>,
    pub reset: KeyBind,

    pub server: ServerSettings,
    pub web: WebSettings,

    pub toml_settings: Settings,
}

impl OverlaySettings {
    pub fn is_changed(one: Self, two: Self) -> bool {
        helper::hash_of(one) != helper::hash_of(two)
    }

    pub fn is_fatal_change(one: Self, two: Self) -> bool {
        helper::hash_of(one.server) != helper::hash_of(two.server)
            || helper::hash_of(one.web) != helper::hash_of(two.web)
    }

    pub fn to_toml(&self) -> anyhow::Result<String> {
        let new_toml_settings = Settings::from(self.clone());

        match toml::to_string_pretty(&new_toml_settings) {
            Ok(toml) => return Ok(toml),
            Err(error) => anyhow::bail!("{:?}", error),
        };
    }
}

impl From<Settings> for OverlaySettings {
    fn from(toml_settings: Settings) -> Self {
        let mut keys: Vec<KeyBind> = vec![];

        for str in &toml_settings.keyboard.keys {
            match KeyBind::deserialize(str.clone()) {
                Ok(key) => keys.push(key),

                Err(()) => {
                    keys = vec![]; // remove keys if there is an error

                    break;
                }
            }
        }

        let mut reset = KeyBind::empty();

        if let Ok(key) = KeyBind::deserialize(toml_settings.keyboard.reset.clone()) {
            reset = key;
        }

        Self {
            keys,
            reset,

            server: toml_settings.server.clone(),
            web: toml_settings.web.clone(),

            toml_settings,
        }
    }
}
