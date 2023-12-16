use std::env;
use std::error::Error;
use std::fs::File;

use itertools::Itertools;
use memmap::MmapOptions;
use parser_sf2::Sf2SoundFont;

fn main() -> Result<(), Box<dyn Error>> {
    let sf2_path = env::args().nth(1).expect("No input file argument");
    let sf2_file = File::open(sf2_path).expect("Failed to open input file");

    let sf2_mmap: &[u8] = unsafe {
        &MmapOptions::new()
            .map(&sf2_file)
            .expect("Failed to mmap input file")
    };

    let sf2_soundfont = Sf2SoundFont::new(sf2_mmap)?;

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
        .iter()
        .sorted_by_key(|phdr| phdr.bank_preset())
    {
        println!(
            "PRESET HEADER [{:3}:{:3}] {}",
            preset_header.bank(),
            preset_header.preset(),
            preset_header.preset_name()?,
        )
    }

    for preset_zone in sf2_soundfont.preset_zones()? {
        println!(
            "PRESET ZONE {} {}",
            preset_zone.generator_index, preset_zone.modulator_index
        )
    }

    for instrument in sf2_soundfont.instruments()? {
        println!(
            "INSTRUMENT {:5} {}",
            instrument.instrument_bag_index,
            instrument.instrument_name()?,
        )
    }

    for instrument_zone in sf2_soundfont.instrument_zones()? {
        println!(
            "INSTRUMENT ZONE {} {}",
            instrument_zone.generator_index, instrument_zone.modulator_index
        )
    }

    for sample in sf2_soundfont.samples()? {
        println!("SAMPLE {}", sample.sample_name()?,)
    }

    Ok(())
}
