use std::error::Error;
use std::fs::File;
use std::{env, str};

use memmap::MmapOptions;
use sf2lib::riff::RiffChunk;
use sf2lib::sf2::Sf2PresetHeader;
use zerocopy::FromBytes;

fn main() -> Result<(), Box<dyn Error>> {
    let sf2_path = env::args().nth(1).expect("No input file argument");
    let sf2_file = File::open(sf2_path).expect("Failed to open input file");

    let sf2_mmap: &[u8] = unsafe {
        &MmapOptions::new()
            .map(&sf2_file)
            .expect("Failed to mmap input file")
    };

    let sfbk = RiffChunk::new(sf2_mmap)?;
    if let Some(info) = sfbk.subchunk("INFO")? {
        for chunk in info.subchunks()? {
            let data = str::from_utf8(chunk.chunk_data()?)?;
            println!("{}: {}", chunk.chunk_id(), data);
        }
    }

    if let Some(pdta) = sfbk.subchunk("pdta")? {
        if let Some(phdr) = pdta.subchunk("phdr")? {
            println!("{}", phdr.chunk_data()?.len());

            let preset_header = Sf2PresetHeader::read_from_prefix(phdr.chunk_data()?);
            println!("{:?}", preset_header);
        }
    }

    Ok(())
}
