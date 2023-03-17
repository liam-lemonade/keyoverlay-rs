extern crate eframe;
extern crate egui;
extern crate egui_keybinds;
extern crate native_dialog;

use std::io::Write;

use anyhow::Context;
use egui::{
    vec2, Align, CentralPanel, Color32, Layout, RichText, ScrollArea, TextEdit, Ui, Widget,
};
use egui_keybinds::{KeyBind, KeyBindWidget};
use native_dialog::FileDialog;

use crate::{
    error::{self, ErrorStatus},
    helper,
    settings::OverlaySettings,
};

struct Gui {
    current_settings: OverlaySettings,
    saved_settings: OverlaySettings,
    used_settings: OverlaySettings, // settings used to initialize the server

    dirty: bool,
    needs_restart: bool,

    current_toml: String,
    saved_toml: String,

    port_str: String,
}

impl Gui {
    pub fn new(settings: OverlaySettings) -> anyhow::Result<Self> {
        Ok(Self {
            current_settings: settings.clone(),
            saved_settings: settings.clone(),
            used_settings: settings.clone(),

            dirty: false,
            needs_restart: false,

            current_toml: settings.to_toml().unwrap(),
            saved_toml: settings.to_toml().unwrap(),

            port_str: settings.server.port.to_string(),
        })
    }
}

impl Gui {
    fn build_tomls(&mut self) -> anyhow::Result<()> {
        self.current_toml = self.current_settings.to_toml()?;
        self.saved_toml = self.saved_settings.to_toml()?;

        Ok(())
    }

    fn draw_left_static(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
            ui.set_enabled(self.dirty);

            if ui.button("Save").clicked() {
                self.saved_settings = self.current_settings.clone();
                self.saved_toml = self.current_toml.clone();

                if let Ok(mut file) = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(crate::SETTINGS_FILENAME)
                {
                    let _ = file.write_all(self.current_toml.as_bytes());
                }
            }
        });
    }

    fn draw_left_scrolled(&mut self, ui: &mut Ui) {
        ui.collapsing("Server", |ui| {
            ui.horizontal(|h| {
                h.label("IP Address:");

                h.add_sized(
                    vec2(100_f32, 20_f32),
                    TextEdit::singleline(&mut self.current_settings.server.ip)
                        .hint_text(self.used_settings.server.ip.clone()),
                )
            });

            ui.horizontal(|h| {
                h.label("Port:");

                h.add_sized(
                    vec2(50_f32, 20_f32),
                    TextEdit::singleline(&mut self.port_str)
                        .hint_text(self.used_settings.server.port.to_string()),
                );

                self.port_str = self.port_str.trim_start_matches("0").to_string();
                if let Ok(port) = self.port_str.parse::<u16>() {
                    self.current_settings.server.port = port;
                } else {
                    self.port_str = self.current_settings.server.port.to_string();
                }
            });
        });

        ui.collapsing("Web", |ui| {
            ui.horizontal(|h| {
                h.label("Websocket Endpoint:");

                h.add_sized(
                    vec2(50_f32, 20_f32),
                    TextEdit::singleline(&mut self.current_settings.web.websocket_endpoint)
                        .hint_text(self.used_settings.web.websocket_endpoint.clone()),
                )
            });

            ui.label(format!(
                "Local files are currently located at: {}",
                self.current_settings.web.local_file_path
            ));

            if ui.button("Select Path").clicked() {
                match FileDialog::new().show_open_single_dir() {
                    Ok(result) => {
                        if let Some(pathbuf) = result {
                            if let Ok(path) = pathbuf.into_os_string().into_string() {
                                self.current_settings.web.local_file_path = path;
                            }
                        }
                    }

                    _ => (),
                }
            }
        });

        ui.collapsing("Keyboard", |ui| {
            ui.collapsing("Keys", |ui| {
                self.current_settings.keys.retain_mut(|mut key| {
                    let mut was_deleted = false;

                    ui.horizontal(|h| {
                        KeyBindWidget::new(&mut key).ui(h);
                        was_deleted = h.button("-").clicked();
                    });

                    return !was_deleted;
                });

                if ui.button("+").clicked() {
                    self.current_settings.keys.push(KeyBind::empty());
                }
            });
        });
    }

    fn draw_right_static(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
            ui.horizontal(|h| {
                if h.button("Quit").clicked() {
                    error::shutdown(ErrorStatus::SUCCESS);
                }

                let address = format!(
                    "http://{}:{}",
                    self.used_settings.server.ip, self.used_settings.server.port
                );

                h.hyperlink_to("Open in Browser", address);
            });

            if self.needs_restart {
                ui.label(
                    RichText::new("Changes have been made that require a restart")
                        .color(Color32::RED),
                );
            }
        });
    }

    fn draw_right_scrolled(&mut self, ui: &mut Ui) {
        ui.collapsing("Current Settings", |ui| {
            ui.set_enabled(false);
            ui.code_editor(&mut self.current_toml);
        });

        ui.collapsing(crate::SETTINGS_FILENAME, |ui| {
            ui.set_enabled(false);
            ui.code_editor(&mut self.saved_toml);
        });
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.dirty = self.current_toml != self.saved_toml;

        self.needs_restart =
            OverlaySettings::is_fatal_change(&self.saved_settings, &self.used_settings);

        let pre_run_hash = helper::hash_of(&self.current_settings);

        CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |col| {
                // push id so scroll areas don't interfere with eachother
                col[0].push_id(1, |ui| {
                    // left
                    ScrollArea::vertical()
                        .max_height(305_f32)
                        .show(ui, |scroll| self.draw_left_scrolled(scroll));

                    self.draw_left_static(ui);
                });

                col[1].push_id(2, |ui| {
                    let scroll_height = if self.needs_restart { 290_f32 } else { 305_f32 };

                    // right
                    ScrollArea::vertical()
                        .max_height(scroll_height)
                        .show(ui, |scroll| self.draw_right_scrolled(scroll));

                    self.draw_right_static(ui);
                });
            })
        });

        // use pre_run_hash variable to detect changes without using is_dirty
        // produces nicer code
        let changed_this_frame = helper::hash_of(&self.current_settings) != pre_run_hash;

        if changed_this_frame {
            let _ = self.build_tomls();
        }
    }
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
