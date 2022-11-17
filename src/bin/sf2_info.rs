use std::env;
use std::error::Error;
use std::fs::File;

use itertools::Itertools;
use memmap::MmapOptions;
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

    let sf2_info = sf2_soundfont.info()?;

    println!("{:?}", sf2_info.format_version()?);
    println!("{:?}", sf2_info.rom_version()?);

    println!("{}", sf2_info.sound_engine()?);
    println!("{}", sf2_info.soundfont_name()?);
    println!("{:?}", sf2_info.rom_name()?);
    println!("{:?}", sf2_info.date()?);
    println!("{:?}", sf2_info.author()?);
    println!("{:?}", sf2_info.product()?);
    println!("{:?}", sf2_info.copyright()?);
    println!("{:?}", sf2_info.comment()?);
    println!("{:?}", sf2_info.soundfont_tools()?);

    for preset_header in sf2_soundfont
        .preset_headers()?
        .sorted_by_key(Sf2PresetHeader::bank_preset)
    {
        println!(
            "[{:3}:{:3}] {}",
            preset_header.bank(),
            preset_header.preset(),
            preset_header.preset_name()?,
        )
    }

    Ok(())
}
