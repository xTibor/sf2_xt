#![feature(cstr_from_bytes_until_nul)]

use std::env;
use std::error::Error;
use std::ffi::CStr;
use std::fs::File;

use itertools::Itertools;
use memmap::MmapOptions;
use sf2lib::sf2::Sf2Soundfont;

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
        .sorted_by_key(|preset_header| (preset_header.bank.get(), preset_header.preset.get()))
    {
        println!(
            "[{:3}:{:3}] {}",
            preset_header.bank,
            preset_header.preset,
            // preset_name may contain garbage after the zero-terminator (GeneralUser GS)
            CStr::from_bytes_until_nul(&preset_header.preset_name)?.to_str()?,
        )
    }

    Ok(())
}
