use std::{fs::File, path::Path};

use memmap::MmapOptions;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub enum RiffChunk<'a> {
    Container {
        identifier: &'a [u8],
        chunk_data: &'a [u8],
        chunk_type: &'a [u8],
    },
    Normal {
        identifier: &'a [u8],
        chunk_data: &'a [u8],
    },
}

impl<'a> RiffChunk<'a> {
    pub fn subchunks(&self) -> RiffChunkIterator<'a> {
        match *self {
            RiffChunk::Container { chunk_data, .. } => RiffChunkIterator::new(chunk_data),
            RiffChunk::Normal { .. } => panic!("Normal chunks cannot have subchunks"),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct RiffChunkIterator<'a> {
    buffer: &'a [u8],
    i: usize,
}

impl<'a> RiffChunkIterator<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, i: 0 }
    }
}

impl<'a> Iterator for RiffChunkIterator<'a> {
    type Item = RiffChunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i + 8 <= self.buffer.len() {
            let identifier = &self.buffer[self.i..self.i + 4];

            let chunk_size =
                u32::from_le_bytes(self.buffer[self.i + 4..self.i + 8].try_into().unwrap());

            let has_subchunks = (identifier == &[b'R', b'I', b'F', b'F'])
                || (identifier == &[b'L', b'I', b'S', b'T']);

            if has_subchunks {
                let chunk_type = &self.buffer[self.i + 8..self.i + 12];
                let chunk_data = &self.buffer[self.i + 12..self.i + 12 + ((chunk_size as usize) - 4)];
                self.i = self.i + 12 + ((chunk_size as usize) - 4);

                Some(RiffChunk::Container {
                    identifier,
                    chunk_data,
                    chunk_type,
                })
            } else {
                let chunk_data = &self.buffer[self.i + 8..self.i + 8 + (chunk_size as usize)];
                self.i = self.i + 8 + (chunk_size as usize);

                Some(RiffChunk::Normal {
                    identifier,
                    chunk_data,
                })
            }
        } else {
            None
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn dump_riff_structure<'a>(chunk: RiffChunk<'a>, chunk_level: usize) {
    match chunk {
        RiffChunk::Container {
            identifier,
            chunk_data,
            chunk_type,
        } => {
            println!(
                "{:chunk_level$}[{}] {} ({})",
                "",
                String::from_utf8_lossy(identifier),
                String::from_utf8_lossy(chunk_type),
                chunk_data.len()
            );

            for chunk in chunk.subchunks() {
                dump_riff_structure(chunk, chunk_level + 1);
            }
        }
        RiffChunk::Normal {
            identifier,
            chunk_data,
        } => {
            println!(
                "{:chunk_level$}{} ({})",
                "",
                String::from_utf8_lossy(identifier),
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

    for chunk in RiffChunkIterator::new(sf2_soundfont_bin) {
        dump_riff_structure(chunk, 0);
    }
}
