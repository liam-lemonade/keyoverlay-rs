extern crate anyhow;
extern crate eframe;
extern crate egui;
extern crate open;

use std::io::Write;

use anyhow::Result;
use egui::*;

use crate::error::ErrorCode;
use crate::{error, server};
use crate::{Settings, SETTINGS};

struct Gui {
    new_settings: Settings,
    old_settings: Settings,

    port_str: String,

    client_count: usize,
    needs_restart: bool,
    will_need_restart: bool,

    dirty: bool,

    old_toml: String,
    new_toml: String,
}

impl Gui {
    fn new() -> anyhow::Result<Self> {
        let settings = (*SETTINGS.read().unwrap()).clone();

        let old_toml = toml::to_string_pretty(&settings)?;
        let new_toml = old_toml.clone();

        let port_str = (&settings).server.port.to_string();

        Ok(Self {
            new_settings: settings.clone(),
            old_settings: settings,

            port_str,

            client_count: 0_usize,
            needs_restart: false,
            will_need_restart: false,

            dirty: false,

            old_toml,
            new_toml,
        })
    }
}

impl Gui {
    fn build_tomls(&mut self) -> Result<()> {
        self.old_toml = toml::to_string_pretty(&self.old_settings)?;
        self.new_toml = toml::to_string_pretty(&self.new_settings)?;

        Ok(())
    }

    fn draw_left_scrolled(&mut self, column: &mut Ui) {
        column.label("keyoverlay-rs configuration");
        column.separator();

        column.collapsing("Files", |ui| {
            ui.label(format!(
                "Static files are currently located at: {}",
                self.new_settings.static_path.clone()
            ));

            if ui.button("Press here to select a new path").clicked() {
                if let Some(path) = tinyfiledialogs::select_folder_dialog(
                    "Select a folder",
                    &self.new_settings.static_path,
                ) {
                    self.new_settings.static_path = path;
                }
            }

            if self.new_settings.static_path != self.old_settings.static_path {
                self.dirty = true;
                self.will_need_restart = true;
            }
        });

        column.collapsing("Server", |ui| {
            ui.horizontal(|h| {
                h.label("Server IP:");

                h.add_sized(
                    vec2(100_f32, 20_f32),
                    TextEdit::singleline(&mut self.new_settings.server.ip)
                        .hint_text(self.old_settings.server.ip.clone()),
                );

                if self.new_settings.server.ip != self.old_settings.server.ip {
                    self.dirty = true;
                    self.will_need_restart = true;
                }
            });

            ui.horizontal(|h| {
                h.label("Port:");

                h.add_sized(
                    vec2(50_f32, 20_f32),
                    TextEdit::singleline(&mut self.port_str)
                        .hint_text(self.old_settings.server.port.to_string()),
                );

                self.port_str = self.port_str.trim_start_matches("0").to_string(); // removing any padding zeros, eg. 00005
                if let Ok(port) = self.port_str.parse::<u16>() {
                    self.new_settings.server.port = port;
                } else {
                    // use new settings so we can revert to most recent
                    self.port_str = self.new_settings.server.port.to_string();
                }

                if self.new_settings.server.port != self.old_settings.server.port {
                    self.dirty = true;
                    self.will_need_restart = true;
                }
            });
        });

        column.collapsing("Keyboard", |ui| {
            ui.collapsing("Main", |ui| {
                self.new_settings.keyboard.keys.retain_mut(|key| {
                    let mut result = true;

                    ui.horizontal(|h| {
                        h.add_sized(
                            vec2(40_f32, 20_f32),
                            TextEdit::singleline(key).hint_text("..."),
                        );

                        if h.button("-").clicked() {
                            result = false;
                        }
                    });

                    return result; // false to remove key from list
                });

                if ui.button("+").clicked() {
                    self.new_settings.keyboard.keys.push(String::new());
                }

                if self.new_settings.keyboard.keys != self.old_settings.keyboard.keys {
                    self.dirty = true;
                }
            });

            ui.horizontal(|h| {
                h.label("Reset:");
                h.add_sized(
                    vec2(40_f32, 20_f32),
                    TextEdit::singleline(&mut self.new_settings.keyboard.reset)
                        .hint_text(self.old_settings.keyboard.reset.clone()),
                );

                if self.new_settings.keyboard.reset != self.old_settings.keyboard.reset {
                    self.dirty = true;
                }
            });
        });
    }

    fn draw_left_static(&mut self, column: &mut Ui) {
        column.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
            ui.push_id(1, |ui| {
                ui.set_enabled(self.dirty);

                if ui.button("Save Configuration").clicked() {
                    {
                        // put this in a scope so we don't deadlock
                        let mut writer = SETTINGS.write().unwrap();

                        *writer = self.new_settings.clone();

                        self.old_settings = self.new_settings.clone();
                        self.old_toml = self.new_toml.clone();

                        if let Ok(mut file) = std::fs::OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(crate::CONFIG_NAME)
                        {
                            let _ = file.write_all(self.new_toml.as_bytes()); // handle error later
                        }
                    }

                    // force refresh the keymap
                    let _ = crate::keyboard::build_keymap();

                    if self.will_need_restart {
                        self.needs_restart = true;
                    }
                }
            });

            ui.label(format!("Connected clients: {:?}", self.client_count));
        });
    }

    fn draw_right_scrolled(&mut self, column: &mut Ui) {
        column.collapsing("Current Configuration", |ui| {
            ui.code_editor(&mut self.new_toml);
        });

        column.collapsing(crate::CONFIG_NAME, |ui| {
            ui.set_enabled(false);
            ui.code_editor(&mut self.old_toml);
        });
    }

    fn draw_right_static(&mut self, column: &mut Ui) {
        column.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
            ui.horizontal(|h| {
                if h.button("Quit").clicked() {
                    error::shutdown(ErrorCode::Success);
                }

                if h.button("Open in Browser").clicked() {
                    let server = &self.old_settings.server;

                    let address = format!("http://{}:{}", server.ip, server.port);
                    let _ = open::that(address);
                }
            });

            if self.needs_restart {
                ui.label(
                    RichText::new("Some changes have been made that require a restart")
                        .color(Color32::RED),
                );
            }
        });
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        {
            // in scope to avoid locking for the entire frame
            self.client_count = server::CLIENT_LIST.lock().unwrap().len();
        }

        self.dirty = false;
        self.will_need_restart = false;

        CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                let normal_scroll_height = 305_f32;

                // 0 == left, 1 == right
                columns[0].push_id(1, |ui| {
                    ScrollArea::vertical()
                        .max_height(normal_scroll_height)
                        .show(ui, |ui| {
                            self.draw_left_scrolled(ui);
                        });
                    self.draw_left_static(ui);
                });

                columns[1].push_id(2, |ui| {
                    let scroll_height = if self.needs_restart {
                        290_f32
                    } else {
                        normal_scroll_height
                    };

                    ScrollArea::vertical()
                        .max_height(scroll_height)
                        .show(ui, |ui| {
                            self.draw_right_scrolled(ui);
                        });
                    self.draw_right_static(ui);
                });

                if self.dirty {
                    let _ = self.build_tomls(); // Handle this later
                }
            });
        });
    }
}

pub fn start() -> Result<()> {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(vec2(650_f32, 350_f32)),
        follow_system_theme: true,
        ..Default::default()
    };

    let gui = Gui::new()?;
    eframe::run_native("keyoverlay-rs", options, Box::new(|_| Box::new(gui)));

    Ok(())
}
