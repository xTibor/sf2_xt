use std::fs::File;
use std::{env, str};

use memmap::MmapOptions;
use sf2lib::riff::{RawChunk, RawChunkIterator};

pub fn print_riff_chunk(chunk: &RawChunk, chunk_level: usize) {
    match chunk {
        RawChunk::Container {
            chunk_type,
            chunk_id,
            chunk_data,
        } => {
            println!(
                "{:chunk_level$}{} [{}] ({})",
                "",
                str::from_utf8(chunk_id).unwrap().escape_default(),
                str::from_utf8(chunk_type).unwrap().escape_default(),
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
    let riff_path = env::args().nth(1).expect("No input file argument");
    let riff_file = File::open(riff_path).expect("Failed to open input file");

    let riff_binary: &[u8] = unsafe {
        &MmapOptions::new()
            .map(&riff_file)
            .expect("Failed to mmap input file")
    };

    if let Some(riff_root) = RawChunkIterator::new(riff_binary).next() {
        print_riff_chunk(&riff_root, 0);
    }
}
