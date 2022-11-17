use std::env;
use std::error::Error;
use std::fs::File;

use itertools::Itertools;
use memmap::MmapOptions;
use sf2lib::gm;
use sf2lib::sf2::{Sf2PresetHeader, Sf2Soundfont};

fn main() -> Result<(), Box<dyn Error>> {
    let sf2_path = env::args().nth(1).expect("No input file argument");
    let sf2_file = File::open(sf2_path).expect("Failed to open input file");

    let sf2_mmap: &[u8] = unsafe {
        &MmapOptions::new()
            .map(&sf2_file)
            .expect("Failed to mmap input file")
    };

    let sf2_soundfont = Sf2Soundfont::new(sf2_mmap)?;

    for preset_header in sf2_soundfont
        .preset_headers()?
        .sorted_by_key(Sf2PresetHeader::bank_preset)
    {
        let gm_name = gm::GENERAL_MIDI
            .binary_search_by_key(&preset_header.bank_preset(), |&(bank_preset, _)| {
                bank_preset
            });

        println!(
            "[{:3}:{:3}] {}",
            preset_header.bank(),
            preset_header.preset(),
            preset_header.preset_name()?,
        )
    }

    Ok(())
}
