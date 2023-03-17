use egui_keybinds::KeyBind;

use crate::settings::OverlaySettings;

fn build_keymap(keys: Vec<KeyBind>, reset: KeyBind) -> anyhow::Result<()> {
    Ok(())
}

pub fn start(settings: OverlaySettings) -> anyhow::Result<()> {
    self::build_keymap(settings.keys, settings.reset)?;

    Ok(())
}
