use std::env;
use std::fs::File;

use memmap::MmapOptions;
use sf2lib::riff::{RiffChunk, RiffResult};

pub fn print_riff_chunk(chunk: &RiffChunk, chunk_level: usize) -> RiffResult<()> {
    match chunk {
        RiffChunk::Container {
            chunk_type,
            chunk_id,
            subchunks,
        } => {
            println!(
                "{:chunk_level$}{} [{}]",
                "",
                chunk_id.escape_default(),
                chunk_type.escape_default(),
            );

            for chunk in subchunks {
                print_riff_chunk(&chunk, chunk_level + 1)?;
            }
        }
        RiffChunk::Normal {
            chunk_id,
            chunk_data,
        } => {
            println!(
                "{:chunk_level$}{} ({})",
                "",
                chunk_id.escape_default(),
                chunk_data.len()
            );
        }
    }

    Ok(())
}

fn main() {
    let riff_path = env::args().nth(1).expect("No input file argument");
    let riff_file = File::open(riff_path).expect("Failed to open input file");

    let riff_binary: &[u8] = unsafe {
        &MmapOptions::new()
            .map(&riff_file)
            .expect("Failed to mmap input file")
    };

    if let Ok(riff_root) = RiffChunk::new(riff_binary) {
        let _ = print_riff_chunk(&riff_root, 0);
    }
}
