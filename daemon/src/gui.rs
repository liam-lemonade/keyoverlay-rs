extern crate anyhow;
extern crate eframe;
extern crate egui;

use std::sync::mpsc::{self, Receiver, Sender};

use anyhow::{Context, Result};
use egui::{vec2, RichText};

use crate::{
    error::{self, ExitStatus},
    settings::Settings,
};

#[derive(Clone, Copy)]
enum GuiEvent {
    ConnectionsUpdate(i32),
}

struct Gui {
    settings: Settings,
}

impl Gui {
    fn new(settings: Settings) -> Self {
        Self { settings }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.collapsing(self.settings.get_name(), |collapsing| {
                collapsing.code_editor(&mut self.settings.raw_json().unwrap());
            });
        });
    }
}

pub fn start_gui(settings: Settings) -> Result<()> {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::Vec2 {
            x: 650_f32,
            y: 350_f32,
        }),
        follow_system_theme: true,
        ..Default::default()
    };

    eframe::run_native(
        "keyoverlay-rs",
        options,
        Box::new(|_| Box::new(Gui::new(settings))),
    );

    Ok(())
}
