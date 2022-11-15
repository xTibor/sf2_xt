use std::{env, fs::File, error::Error, str};

use memmap::MmapOptions;
use sf2lib::riff::RiffChunk;

fn main() -> Result<(), Box<dyn Error>> {
    let sf2_path = env::args().nth(1).expect("No input file argument");
    let sf2_file = File::open(sf2_path).expect("Failed to open input file");

    let sf2_binary: &[u8] = unsafe {
        &MmapOptions::new()
            .map(&sf2_file)
            .expect("Failed to mmap input file")
    };

    let sfbk = RiffChunk::new(sf2_binary)?;
    if let Some(info) = sfbk.subchunk("INFO")? {
        for chunk in info.subchunks()? {
            let data = str::from_utf8(chunk.chunk_data()?)?;
            println!("{}: {}", chunk.chunk_id(), data);
        }
    }

    Ok(())
}
