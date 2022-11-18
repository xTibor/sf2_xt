use std::env;
use std::fs::File;

use eframe::egui::{CentralPanel, Context, Grid, Layout, ScrollArea};
use eframe::emath::{vec2, Align};
use itertools::Itertools;
use memmap::MmapOptions;
use sf2_lib::sf2::{Sf2PresetHeader, Sf2Soundfont};

struct Sf2GuiApp {
    preset_headers: Vec<((u16, u16), String)>,
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

        Self { preset_headers }
    }
}

impl eframe::App for Sf2GuiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                Grid::new("presets")
                    .num_columns(3)
                    .spacing([20.0, 10.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.strong("Bank")
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.strong("Preset")
                        });
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.strong("Name")
                        });
                        ui.end_row();

                        for ((bank, preset), preset_name) in &self.preset_headers {
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(bank.to_string())
                            });
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(preset.to_string())
                            });
                            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                ui.label(preset_name)
                            });
                            ui.end_row();
                        }
                    });
            });
        });
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
