use std::fs::File;
use std::{env, str};

use memmap::MmapOptions;
use sf2lib::riff::{RawChunk, RawChunkIterator};

pub fn print_riff_chunk(chunk: &RawChunk, chunk_level: usize) {
    match chunk {
        RawChunk::Container {
            container_type,
            chunk_id,
            chunk_data,
        } => {
            println!(
                "{:chunk_level$}{} [{}] ({})",
                "",
                str::from_utf8(chunk_id).unwrap().escape_default(),
                str::from_utf8(container_type).unwrap().escape_default(),
                chunk_data.len()
            );

            for chunk in chunk.subchunks() {
                print_riff_chunk(&chunk, chunk_level + 1);
            }
        }
        RawChunk::Normal {
            chunk_id,
            chunk_data,
        } => {
            println!(
                "{:chunk_level$}{} ({})",
                "",
                str::from_utf8(chunk_id).unwrap().escape_default(),
                chunk_data.len()
            );
        }
    }
}

fn main() {
    let file_path = env::args().skip(1).next().expect("No input file argument");

    let sf2_soundfont_bin: &[u8] = &{
        let file = File::open(file_path).expect("Failed to open input file");
        unsafe {
            MmapOptions::new()
                .map(&file)
                .expect("Failed to mmap input file")
        }
    };

    for chunk in RawChunkIterator::new(sf2_soundfont_bin) {
        print_riff_chunk(&chunk, 0);
    }
}
