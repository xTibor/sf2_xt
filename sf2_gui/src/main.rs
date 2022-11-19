use std::fs::File;
use std::path::Path;
use std::{env, mem};

use eframe::egui::{
    CentralPanel, CollapsingHeader, Context, Layout, ScrollArea, SidePanel, TextEdit,
    TopBottomPanel, Ui,
};
use eframe::emath::{vec2, Align};
use egui_extras::{Size, TableBuilder};
use itertools::Itertools;
use memmap::{Mmap, MmapOptions};

use egui_extras_xt::show_about_window;
use sf2_lib::sf2::{Sf2PresetHeader, Sf2Soundfont};

struct Sf2GuiApp<'a> {
    search_query: String,
    about_window_open: bool,
    request_scrollback: bool,

    sf2_mmap: Option<Mmap>,
    sf2_soundfont: Option<Sf2Soundfont<'a>>,
    sf2_sorted_preset_headers: Vec<(Sf2PresetHeader, bool)>,
}

impl<'a> Sf2GuiApp<'a> {
    pub fn new() -> Self {
        Self {
            search_query: "".to_owned(),
            about_window_open: false,
            request_scrollback: false,

            sf2_mmap: None,
            sf2_soundfont: None,
            sf2_sorted_preset_headers: Vec::new(),
        }
    }

    pub fn load_sf2(&mut self, sf2_path: &Path) {
        let sf2_file = File::open(sf2_path).expect("Failed to open input file");

        self.sf2_mmap = Some(unsafe {
            MmapOptions::new()
                .map(&sf2_file)
                .expect("Failed to mmap input file")
        });

        self.sf2_soundfont = Some(unsafe {
            let sf2_mmap_transmuted_lifetime =
                mem::transmute::<&[u8], &[u8]>(&self.sf2_mmap.as_ref().unwrap());
            Sf2Soundfont::new(sf2_mmap_transmuted_lifetime).unwrap()
        });

        self.resort_preset_headers();
    }

    pub fn resort_preset_headers(&mut self) {
        println!("RESORTED");
        if let Some(sf2_soundfont) = &self.sf2_soundfont {
            self.sf2_sorted_preset_headers = sf2_soundfont
                .preset_headers()
                .unwrap()
                .sorted_by_key(|preset_header| preset_header.bank_preset())
                .map(|preset_header| (preset_header, false))
                .collect::<Vec<_>>();
            self.request_scrollback = true;
        }
    }
}

impl<'a> eframe::App for Sf2GuiApp<'a> {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
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
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                });

                if ui.button("About").clicked() {
                    self.about_window_open = true;
                }
            });
        });

        SidePanel::right("info").min_width(200.0).show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                if let Some(sf2_soundfont) = &self.sf2_soundfont {
                    let sf2_info = sf2_soundfont.info().unwrap();

                    fn add_section(
                        ui: &mut Ui,
                        section_name: &str,
                        default_open: bool,
                        section_contents: &str,
                    ) {
                        if !section_contents.trim().is_empty() {
                            CollapsingHeader::new(section_name)
                                .default_open(default_open)
                                .show(ui, |ui| {
                                    ui.label(section_contents);
                                });
                        }
                    }

                    if let Ok(soundfont_name) = sf2_info.soundfont_name() {
                        add_section(ui, "Soundfont name", true, soundfont_name);
                    }

                    if let Ok(Some(author)) = sf2_info.author() {
                        add_section(ui, "Author", true, author);
                    }

                    if let Ok(Some(copyright)) = sf2_info.copyright() {
                        add_section(ui, "Copyright", true, copyright);
                    }

                    if let Ok(Some(comment)) = sf2_info.comment() {
                        add_section(ui, "Comment", true, comment);
                    }

                    if let Ok((major, minor)) = sf2_info.format_version() {
                        add_section(ui, "Format version", true, &format!("{major:}.{minor:02}"));
                    }

                    if let Ok(sound_engine) = sf2_info.sound_engine() {
                        add_section(ui, "Sound engine", true, sound_engine);
                    }

                    if let Ok(Some(rom_name)) = sf2_info.rom_name() {
                        add_section(ui, "ROM name", true, rom_name);
                    }

                    if let Ok(Some((major, minor))) = sf2_info.rom_version() {
                        add_section(ui, "ROM version", true, &format!("{major:}.{minor:02}"));
                    }

                    if let Ok(Some(date)) = sf2_info.date() {
                        add_section(ui, "Date", true, date);
                    }

                    if let Ok(Some(product)) = sf2_info.product() {
                        add_section(ui, "Product", true, product);
                    }

                    if let Ok(Some(soundfont_tools)) = sf2_info.soundfont_tools() {
                        add_section(ui, "Soundfont tools", true, &soundfont_tools.join(", "));
                    }
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
                                    self.resort_preset_headers();
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
                                    self.resort_preset_headers();
                                }
                            });
                        });
                    });
                })
                .body(|mut body| {
                    for (preset_header, matches_search) in &self.sf2_sorted_preset_headers {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.monospace(preset_header.bank().to_string())
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.monospace((preset_header.preset() + 1).to_string());
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                    let preset_symbol =
                                        if [120, 127, 128].contains(&preset_header.bank()) {
                                            "\u{1F941}"
                                        } else {
                                            "\u{1F3B9}"
                                        };

                                    if *matches_search {
                                        ui.strong(format!(
                                            "{preset_symbol:} {preset_name:}",
                                            preset_name = preset_header.preset_name().unwrap()
                                        ));
                                    } else {
                                        ui.label(format!(
                                            "{preset_symbol:} {preset_name:}",
                                            preset_name = preset_header.preset_name().unwrap()
                                        ));
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
        initial_window_size: Some(vec2(500.0, 600.0)),
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
