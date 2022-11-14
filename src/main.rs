#![feature(int_roundings)]

use std::{fs::File, path::Path};

use memmap::MmapOptions;

mod riff;
use riff::{RawChunk, RawChunkIterator};

pub fn dump_riff_structure(chunk: &RawChunk, chunk_level: usize) {
    match chunk {
        RawChunk::Container {
            container_type,
            chunk_id,
            chunk_data,
        } => {
            println!(
                "{:chunk_level$}{} [{}] ({})",
                "",
                String::from_utf8_lossy(chunk_id),
                String::from_utf8_lossy(container_type),
                chunk_data.len()
            );

            for chunk in chunk.subchunks() {
                dump_riff_structure(&chunk, chunk_level + 1);
            }
        }
        RawChunk::Normal {
            chunk_id,
            chunk_data,
        } => {
            println!(
                "{:chunk_level$}{} ({})",
                "",
                String::from_utf8_lossy(chunk_id),
                chunk_data.len()
            );
        }
    }
}

fn main() {
    let sf2_soundfont_bin: &[u8] = &{
        let file_path = Path::new("/home/tibor/Downloads/Music Theory/SF2/Newgrounds - blackattackbitch/Strings/Studio FG460s II Pro Guitar Pack.SF2");
        let file = File::open(file_path).unwrap();
        unsafe { MmapOptions::new().map(&file).unwrap() }
    };

    for chunk in RawChunkIterator::new(sf2_soundfont_bin) {
        dump_riff_structure(&chunk, 0);
    }
}
