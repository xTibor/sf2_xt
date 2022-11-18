use std::env;
use std::fs::File;

use eframe::egui::{CentralPanel, Context, Layout, TopBottomPanel};
use eframe::emath::{vec2, Align};
use egui_extras::{Size, TableBuilder};
use itertools::Itertools;
use memmap::MmapOptions;

use egui_extras_xt::show_about_window;
use sf2_lib::sf2::{Sf2PresetHeader, Sf2Soundfont};

struct Sf2GuiApp {
    preset_headers: Vec<((u16, u16), String)>,
    about_window_open: bool,
}

impl Sf2GuiApp {
    pub fn new(sf2_path: &str) -> Self {
        let sf2_file = File::open(sf2_path).expect("Failed to open input file");
        let sf2_mmap = unsafe {
            MmapOptions::new()
                .map(&sf2_file)
                .expect("Failed to mmap input file")
        };
        let sf2_soundfont = Sf2Soundfont::new(&sf2_mmap).unwrap();

        let preset_headers = sf2_soundfont
            .preset_headers()
            .unwrap()
            .sorted_by_key(Sf2PresetHeader::bank_preset)
            .map(|preset_header| {
                (
                    preset_header.bank_preset(),
                    preset_header.preset_name().unwrap().to_owned(),
                )
            })
            .collect::<Vec<_>>();

        Self {
            preset_headers,
            about_window_open: false,
        }
    }
}

impl eframe::App for Sf2GuiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("mainmenu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("About").clicked() {
                    self.about_window_open = true;
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Size::exact(20.0))
                .column(Size::exact(20.0))
                .column(Size::remainder().at_least(100.0))
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.heading("\u{1F5C0}").on_hover_text("Bank number");
                        });
                    });
                    header.col(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.heading("\u{1F3B5}").on_hover_text("Preset number");
                        });
                    });
                    header.col(|ui| {
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.heading("Preset name");
                        });
                    });
                })
                .body(|mut body| {
                    for ((bank, preset), preset_name) in &self.preset_headers {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.monospace(bank.to_string())
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.monospace((preset + 1).to_string());
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                    ui.label(preset_name);
                                });
                            });
                        });
                    }
                });
        });

        show_about_window!(ctx, &mut self.about_window_open);
    }
}

fn main() {
    let sf2_path = env::args().nth(1).expect("No input file argument");

    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(500.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "sf2_gui",
        options,
        Box::new(move |_| Box::new(Sf2GuiApp::new(&sf2_path))),
    );
}
