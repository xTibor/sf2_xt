use std::env;
use std::fs::File;
use std::path::Path;

use eframe::egui::{CentralPanel, Context, Layout, TextEdit, TopBottomPanel};
use eframe::emath::{vec2, Align};
use egui_extras::{Size, TableBuilder};
use itertools::Itertools;
use memmap::MmapOptions;

use egui_extras_xt::show_about_window;
use sf2_lib::sf2::{Sf2PresetHeader, Sf2Soundfont};

struct Sf2GuiApp {
    preset_headers: Vec<((u16, u16), String)>,
    search_query: String,
    about_window_open: bool,
    request_scrollback: bool,
}

impl Sf2GuiApp {
    pub fn new() -> Self {
        Self {
            preset_headers: Vec::new(),
            search_query: "".to_owned(),
            about_window_open: false,
            request_scrollback: false,
        }
    }

    pub fn load_sf2(&mut self, sf2_path: &Path) {
        let sf2_file = File::open(sf2_path).expect("Failed to open input file");
        let sf2_mmap = unsafe {
            MmapOptions::new()
                .map(&sf2_file)
                .expect("Failed to mmap input file")
        };
        let sf2_soundfont = Sf2Soundfont::new(&sf2_mmap).unwrap();

        self.preset_headers = sf2_soundfont
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

        self.request_scrollback = true;
    }
}

impl eframe::App for Sf2GuiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !ctx.input().raw.dropped_files.is_empty() {
            if let Some(sf2_path) = ctx
                .input()
                .raw
                .dropped_files
                .first()
                .and_then(|f| f.path.as_ref())
            {
                self.load_sf2(sf2_path);
            }
        }

        TopBottomPanel::top("mainmenu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("About").clicked() {
                    self.about_window_open = true;
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            let mut table_builder = TableBuilder::new(ui)
                .striped(true)
                .column(Size::exact(20.0))
                .column(Size::exact(20.0))
                .column(Size::remainder().at_least(100.0))
                .column(Size::exact(20.0));

            if self.request_scrollback {
                // Uncomment when egui 0.20.0 releases
                //table_builder = table_builder.vertical_scroll_offset(0.0);
                self.request_scrollback = false;
            }

            table_builder
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.heading("\u{1F5C0}")
                                .on_hover_text("\u{1F5C0} Bank number");
                        });
                    });
                    header.col(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.heading("\u{1F3B5}")
                                .on_hover_text("\u{1F3B5} Preset number");
                        });
                    });
                    header.col(|ui| {
                        ui.with_layout(
                            Layout::left_to_right(Align::Center).with_main_justify(true),
                            |ui| {
                                if ui
                                    .add(
                                        TextEdit::singleline(&mut self.search_query)
                                            .hint_text("\u{1F50D} Preset name"),
                                    )
                                    .changed()
                                {
                                    self.request_scrollback = true;
                                }
                            },
                        );
                    });
                    header.col(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.add_enabled_ui(!self.search_query.is_empty(), |ui| {
                                if ui
                                    .button("\u{1F5D9}")
                                    .on_hover_text("\u{1F5D9} Clear search query")
                                    .clicked()
                                {
                                    self.search_query.clear();
                                    self.request_scrollback = true;
                                }
                            });
                        });
                    });
                })
                .body(|mut body| {
                    for ((bank, preset), preset_name, matches_search) in self
                        .preset_headers
                        .iter()
                        .map(|((bank, preset), preset_name)| {
                            let matches_search = if self.search_query.is_empty() {
                                false
                            } else {
                                let preset_name_match = preset_name
                                    .to_lowercase()
                                    .contains(&self.search_query.to_lowercase());

                                let bank_preset_match = if self.search_query.contains(':')
                                    && self.search_query.len() > 1
                                {
                                    format!("{}:{}", bank, preset + 1).contains(&self.search_query)
                                } else {
                                    false
                                };

                                preset_name_match || bank_preset_match
                            };

                            ((bank, preset), preset_name, matches_search)
                        })
                        .sorted_by_key(|((bank, preset), _, matches_search)| {
                            (!*matches_search, (*bank, *preset))
                        })
                    {
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
                                    let preset_symbol = if [120, 127, 128].contains(bank) {
                                        "\u{1F941}"
                                    } else {
                                        "\u{1F3B9}"
                                    };

                                    if matches_search {
                                        ui.strong(format!("{preset_symbol:} {preset_name:}"));
                                    } else {
                                        ui.label(format!("{preset_symbol:} {preset_name:}"));
                                    }
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let _ = ui.button("\u{23F5}");
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
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(300.0, 600.0)),
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(
        "sf2_gui",
        options,
        Box::new(|_| {
            let mut app = Sf2GuiApp::new();

            if let Some(sf2_path) = env::args().nth(1) {
                app.load_sf2(Path::new(&sf2_path))
            }

            Box::new(app)
        }),
    );
}
