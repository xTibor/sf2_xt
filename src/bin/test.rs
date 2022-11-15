use std::env;
use std::fs::File;

use memmap::MmapOptions;
use sf2lib::riff::RiffChunk;

fn main() {
    let riff_path = env::args().nth(1).expect("No input file argument");
    let riff_file = File::open(riff_path).expect("Failed to open input file");

    let riff_binary: &[u8] = unsafe {
        &MmapOptions::new()
            .map(&riff_file)
            .expect("Failed to mmap input file")
    };

    let riff_chunk = RiffChunk::new(riff_binary);
    println!("{:#?}", riff_chunk);
}
