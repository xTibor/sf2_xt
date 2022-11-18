use std::env;
use std::fs::File;

use eframe::egui::{CentralPanel, Context};
use eframe::emath::vec2;
use memmap::{Mmap, MmapOptions};
use sf2_lib::sf2::Sf2Soundfont;

struct Sf2GuiApp<'a> {
    sf2_file: File,
    sf2_mmap: Mmap,
    sf2_soundfont: Sf2Soundfont<'a>,
}

impl<'a> Sf2GuiApp<'a> {
    pub fn new(sf2_path: &str) -> Self {
        let sf2_file = File::open(sf2_path).expect("Failed to open input file");
        let sf2_mmap = unsafe {
            MmapOptions::new()
                .map(&sf2_file)
                .expect("Failed to mmap input file")
        };
        let sf2_soundfont = Sf2Soundfont::new(&sf2_mmap).unwrap();

        Self {
            sf2_file,
            sf2_mmap,
            sf2_soundfont,
        }
    }
}

impl<'a> eframe::App for Sf2GuiApp<'a> {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {});
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
