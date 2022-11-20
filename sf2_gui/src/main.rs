use std::cmp::Ordering;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, mem};

use itertools::Itertools;
use memmap::{Mmap, MmapOptions};
use strum::{Display, EnumIter, IntoEnumIterator};

use eframe::egui::{
    CentralPanel, CollapsingHeader, Context, Layout, ScrollArea, SidePanel, TextEdit, TextStyle,
    TopBottomPanel, Ui,
};
use eframe::emath::{vec2, Align};
use egui_extras::{Size, TableBuilder};

use egui_extras_xt::show_about_window;
use egui_extras_xt::ui::hyperlink_with_icon::HyperlinkWithIcon;
use egui_extras_xt::ui::widgets_from::WidgetsFromIterator;

use sf2_lib::sf2::{Sf2PresetHeader, Sf2SoundFont};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Copy, Clone, PartialEq, EnumIter, Display)]
enum PresetSortOrder {
    #[strum(to_string = "Name")]
    Name,

    #[strum(to_string = "Bank and preset")]
    BankPreset,
}

impl PresetSortOrder {
    pub fn cmp_preset_headers(
        &self,
        preset_header_a: &Sf2PresetHeader,
        preset_header_b: &Sf2PresetHeader,
    ) -> Ordering {
        match self {
            PresetSortOrder::Name => preset_header_a
                .preset_name
                .cmp(&preset_header_b.preset_name),
            PresetSortOrder::BankPreset => preset_header_a
                .bank_preset()
                .cmp(&preset_header_b.bank_preset()),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct Sf2GuiApp<'a> {
    search_query: String,
    about_window_open: bool,
    request_scrollback: bool,
    preset_sort_order: PresetSortOrder,

    sf2_mmap: Option<Mmap>,
    sf2_soundfont: Option<Sf2SoundFont<'a>>,
    sf2_sorted_preset_headers: Vec<(Sf2PresetHeader, bool)>,
}

impl<'a> Sf2GuiApp<'a> {
    pub fn new() -> Self {
        Self {
            search_query: "".to_owned(),
            about_window_open: false,
            request_scrollback: false,
            preset_sort_order: PresetSortOrder::BankPreset,

            sf2_mmap: None,
            sf2_soundfont: None,
            sf2_sorted_preset_headers: Vec::new(),
        }
    }

    pub fn new_window(&self) {
        Command::new(env::args().nth(0).unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to create new instance");
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
            Sf2SoundFont::new(sf2_mmap_transmuted_lifetime).unwrap()
        });

        self.resort_preset_headers();
    }

    pub fn resort_preset_headers(&mut self) {
        if let Some(sf2_soundfont) = &self.sf2_soundfont {
            let bank_preset_query =
                self.search_query
                    .trim()
                    .split_once(':')
                    .map(|(bank, preset)| {
                        (
                            bank.trim().parse::<u16>().ok(),
                            preset.trim().parse::<u16>().ok(),
                        )
                    });

            self.sf2_sorted_preset_headers = sf2_soundfont
                .preset_headers()
                .unwrap()
                .map(|preset_header| {
                    let matches_search = if let Some((bank, preset)) = bank_preset_query {
                        let any_field_present = bank.is_some() || preset.is_some();

                        let bank_matches = if let Some(bank) = bank {
                            preset_header.bank() == bank
                        } else {
                            true
                        };

                        let preset_matches = if let Some(preset) = preset {
                            preset_header.preset() + 1 == preset
                        } else {
                            true
                        };

                        any_field_present && bank_matches && preset_matches
                    } else {
                        let matches_preset_name = preset_header
                            .preset_name()
                            .unwrap()
                            .to_lowercase()
                            .contains(&self.search_query.to_lowercase());

                        !self.search_query.is_empty() && matches_preset_name
                    };

                    (preset_header, matches_search)
                })
                .sorted_by(
                    |(preset_header_a, matches_search_a), (preset_header_b, matches_search_b)| {
                        let matches_search_ordering =
                            matches_search_a.cmp(matches_search_b).reverse();

                        let preset_header_ordering = self
                            .preset_sort_order
                            .cmp_preset_headers(preset_header_a, preset_header_b);

                        matches_search_ordering.then(preset_header_ordering)
                    },
                )
                .collect::<Vec<_>>();
            self.request_scrollback = true;
        } else {
            self.sf2_sorted_preset_headers = Vec::new();
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
                    if ui.button("New Window").clicked() {
                        self.new_window();
                        ui.close_menu();
                    }

                    ui.separator();

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
                        add_contents: impl FnOnce(&mut Ui),
                    ) {
                        CollapsingHeader::new(section_name)
                            .default_open(default_open)
                            .show(ui, add_contents);
                    }

                    if let Ok(soundfont_name) = sf2_info.soundfont_name() {
                        if !soundfont_name.trim().is_empty() {
                            add_section(ui, "SoundFont name", true, |ui| {
                                ui.label(soundfont_name);
                            });
                        }
                    }

                    if let Ok(Some(author)) = sf2_info.author() {
                        if !author.trim().is_empty() {
                            add_section(ui, "Author", true, |ui| {
                                ui.label(author);
                            });
                        }
                    }

                    if let Ok(Some(copyright)) = sf2_info.copyright() {
                        if !copyright.trim().is_empty() {
                            add_section(ui, "Copyright", true, |ui| {
                                ui.label(copyright);
                            });
                        }
                    }

                    if let Ok(Some(comment)) = sf2_info.comment() {
                        if !comment.trim().is_empty() {
                            add_section(ui, "Comment", true, |ui| {
                                ui.label(comment);
                            });
                        }
                    }

                    if let Ok((major, minor)) = sf2_info.format_version() {
                        add_section(ui, "Format version", true, |ui| {
                            ui.label(&format!("{major:}.{minor:02}"));
                        });
                    }

                    if let Ok(sound_engine) = sf2_info.sound_engine() {
                        if !sound_engine.trim().is_empty() {
                            add_section(ui, "Sound engine", true, |ui| {
                                ui.label(sound_engine);
                            });
                        }
                    }

                    if let Ok(Some(rom_name)) = sf2_info.rom_name() {
                        if !rom_name.trim().is_empty() {
                            add_section(ui, "ROM name", true, |ui| {
                                ui.label(rom_name);
                            });
                        }
                    }

                    if let Ok(Some((major, minor))) = sf2_info.rom_version() {
                        add_section(ui, "ROM version", true, |ui| {
                            ui.label(&format!("{major:}.{minor:02}"));
                        });
                    }

                    if let Ok(Some(date)) = sf2_info.date() {
                        if !date.trim().is_empty() {
                            add_section(ui, "Date", true, |ui| {
                                ui.label(date);
                            });
                        }
                    }

                    if let Ok(Some(product)) = sf2_info.product() {
                        if !product.trim().is_empty() {
                            add_section(ui, "Product", true, |ui| {
                                ui.label(product);
                            });
                        }
                    }

                    if let Ok(Some(soundfont_tools)) = sf2_info.soundfont_tools() {
                        if !soundfont_tools.is_empty() {
                            add_section(ui, "SoundFont tools", true, |ui| {
                                #[rustfmt::skip]
                                pub const SOUNDFONT_TOOLS_URLS: &[(&str, &str)] = &[
                                    ("Polyphone",       "https://www.polyphone-soundfonts.com"    ),
                                    ("SynthFont Viena", "https://www.synthfont.com"               ),
                                    ("CDXtract",        "https://www.soundlib.com/cdxtract/"      ),
                                    ("Awave Studio",    "https://www.fmjsoft.com/awavestudio.html"),
                                    ("SWAMI",           "http://www.swamiproject.org"             ),
                                    ("libInstPatch",    "http://www.swamiproject.org"             ),
                                    //("SFEDT",           "???"                                     ),
                                ];

                                for &soundfont_tool in soundfont_tools.iter().unique() {
                                    if let Some(&(_, soundfont_tool_url)) =
                                        SOUNDFONT_TOOLS_URLS.iter().find(|(name, _)| {
                                            soundfont_tool
                                                .to_lowercase()
                                                .starts_with(&name.to_lowercase())
                                        })
                                    {
                                        ui.hyperlink_with_icon_to(
                                            soundfont_tool,
                                            soundfont_tool_url,
                                        );
                                    } else {
                                        ui.label(format!("\u{1F6E0} {soundfont_tool:}"));
                                    }
                                }
                            });
                        }
                    }
                }
            });
        });

        CentralPanel::default()
            .show(ctx, |ui| {
                if self.sf2_soundfont.is_some() {
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
                                        ui.with_layout(
                                            Layout::right_to_left(Align::Center),
                                            |ui| ui.monospace(preset_header.bank().to_string()),
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(
                                            Layout::right_to_left(Align::Center),
                                            |ui| {
                                                ui.monospace(
                                                    (preset_header.preset() + 1).to_string(),
                                                );
                                            },
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(
                                            Layout::left_to_right(Align::Center),
                                            |ui| {
                                                let preset_symbol = if [120, 127, 128]
                                                    .contains(&preset_header.bank())
                                                {
                                                    "\u{1F941}"
                                                } else {
                                                    "\u{1F3B9}"
                                                };

                                                if *matches_search {
                                                    ui.strong(format!(
                                                        "{preset_symbol:} {preset_name:}",
                                                        preset_name =
                                                            preset_header.preset_name().unwrap()
                                                    ));
                                                } else {
                                                    ui.label(format!(
                                                        "{preset_symbol:} {preset_name:}",
                                                        preset_name =
                                                            preset_header.preset_name().unwrap()
                                                    ));
                                                }
                                            },
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(
                                            Layout::right_to_left(Align::Center),
                                            |ui| {
                                                if ui.button("\u{23F5}").clicked() {
                                                    // TODO
                                                }
                                            },
                                        );
                                    });
                                });
                            }
                        });
                } else {
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.scope(|ui| {
                            ui.style_mut()
                                .text_styles
                                .get_mut(&TextStyle::Body)
                                .unwrap()
                                .size = 100.0;
                            ui.label("\u{2B8B}");
                        });
                        ui.heading("Drop files to open here");
                        ui.label("or pass them as command-line arguments.")
                    });
                }
            })
            .response
            .context_menu(|ui| {
                ui.menu_button("\u{1F4CA} Sort by", |ui| {
                    if ui
                        .radio_value_from_iter(&mut self.preset_sort_order, PresetSortOrder::iter())
                        .changed()
                    {
                        self.resort_preset_headers();
                        ui.close_menu();
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
