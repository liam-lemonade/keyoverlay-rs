use anyhow::Context;
use egui::vec2;

use crate::settings::OverlaySettings;

struct Gui {
    new_settings: OverlaySettings,
    old_settings: OverlaySettings,
}

impl Gui {
    pub fn new(settings: OverlaySettings) -> anyhow::Result<Self> {
        Ok(Self {
            new_settings: settings.clone(),
            old_settings: settings,
        })
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}

pub fn start(settings: OverlaySettings) -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(vec2(650_f32, 350_f32)),
        follow_system_theme: true,
        ..Default::default()
    };

    let gui = Gui::new(settings).with_context(|| "Failed to initialize Gui struct")?;

    if let Err(_) = eframe::run_native(crate::TITLE, options, Box::new(|_| Box::new(gui))) {
        anyhow::bail!("Failed to run eframe native window");
    }

    Ok(())
}
