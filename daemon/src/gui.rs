extern crate anyhow;
extern crate eframe;
extern crate egui;

use std::sync::mpsc::Receiver;

use anyhow::Result;
use egui::vec2;

use crate::{
    error::{self, ExitStatus},
    settings::Settings,
};

#[derive(Clone, Debug)]
pub enum GuiEvent {
    ConnectionsUpdate(usize),
}

struct Gui {
    settings: Settings,
    json: String,
    client_count: usize,
    key_list: Vec<String>,
    needs_restart: bool,

    receiver: Receiver<GuiEvent>,
}

impl Gui {
    fn new(settings: Settings, receiver: Receiver<GuiEvent>) -> Self {
        let json = settings.raw_json().unwrap_or_else(|error| {
            error::handle_error("An error occured while running the gui thread", error);
            error::shutdown(ExitStatus::Failure);
        });

        let keys = settings
            .read_config::<Vec<String>>("keys")
            .unwrap_or_else(|error| {
                error::handle_error("An error occured while running the gui thread", error);
                error::shutdown(ExitStatus::Failure);
            });

        Self {
            settings,
            json,
            client_count: 0,
            key_list: keys,
            needs_restart: false,

            receiver,
        }
    }
}

impl Gui {
    fn process_event(&mut self, event: GuiEvent) {
        match event {
            GuiEvent::ConnectionsUpdate(count) => {
                self.client_count = count;
            }
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.receiver.try_recv() {
            self.process_event(event);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                // left side
                columns[0].label("keyoverlay-rs configurator");
                columns[0].separator();

                egui::ScrollArea::vertical()
                    .max_height(270_f32)
                    .show(&mut columns[0], |ui| {
                        ui.collapsing("Keybinds", |ui| {
                            self.key_list.retain_mut(|key| {
                                let mut return_value = true;

                                ui.horizontal(|ui| {
                                    ui.add_sized(
                                        vec2(20_f32, 20_f32),
                                        egui::TextEdit::singleline(key).hint_text("..."),
                                    );
                                    //ui.text_edit_singleline(key);

                                    return_value = !ui.button("-").clicked();
                                });

                                return_value
                            });

                            if ui.button("+").clicked() {
                                self.key_list.push(String::new());
                            }
                        });
                    });

                columns[0].with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    if ui.button("Save Configuration").clicked() {
                        // save config file
                        if let Ok(old_json) = self.settings.raw_json() {
                            if self.json != old_json {
                                // replace the file

                                self.needs_restart = true;
                            }
                        }
                    }

                    ui.label(format!("Connected clients: {:?}", self.client_count));
                });

                // right side
                columns[1].collapsing(self.settings.get_name(), |collapsing| {
                    collapsing.code_editor(&mut self.json);
                });

                columns[1].with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                    // bottom right buttons
                    ui.horizontal(|ui| {
                        if ui.button("Quit").clicked() {
                            error::shutdown(ExitStatus::Success);
                        }

                        if ui.button("Open in Browser").clicked() {
                            if let Ok(port) = self.settings.read_config::<u16>("web_port") {
                                let address = format!("http://127.0.0.1:{:?}", port);
                                let _ = open::that(&address);
                            }
                        }
                    });

                    if self.needs_restart {
                        ui.label(
                            egui::RichText::new(
                                "Some settings have been changed that require a restart",
                            )
                            .color(egui::Color32::RED),
                        );
                    }
                });
            });
        });
    }
}

pub fn start_gui(settings: Settings, receiver: Receiver<GuiEvent>) -> Result<()> {
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
        Box::new(|_| Box::new(Gui::new(settings, receiver))),
    );

    Ok(())
}
