use crate::{RiffError, RiffResult};

pub enum RawChunk<'a> {
    Container {
        chunk_type: &'a [u8],
        chunk_id: &'a [u8],
        chunk_data: &'a [u8],
    },
    Normal {
        chunk_id: &'a [u8],
        chunk_data: &'a [u8],
    },
}

pub struct RawChunkIterator<'a> {
    buffer: &'a [u8],
    i: usize,
}

impl<'a> RawChunkIterator<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, i: 0 }
    }
}

impl<'a> Iterator for RawChunkIterator<'a> {
    type Item = RiffResult<RawChunk<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i + 8 <= self.buffer.len() {
            let chunk_id = &self.buffer[self.i..self.i + 4];
            let is_container = (chunk_id == b"RIFF") || (chunk_id == b"LIST");

            let chunk_size =
                u32::from_le_bytes(self.buffer[self.i + 4..self.i + 8].try_into().unwrap());

            if self.i + 8 + (chunk_size as usize) <= self.buffer.len() {
                let chunk = if is_container {
                    let chunk_type = chunk_id;
                    let chunk_id = &self.buffer[self.i + 8..self.i + 12];

                    let chunk_data =
                        &self.buffer[self.i + 12..self.i + 12 + ((chunk_size as usize) - 4)];

                    Some(Ok(RawChunk::Container {
                        chunk_type,
                        chunk_id,
                        chunk_data,
                    }))
                } else {
                    let chunk_data = &self.buffer[self.i + 8..self.i + 8 + (chunk_size as usize)];

                    Some(Ok(RawChunk::Normal {
                        chunk_id,
                        chunk_data,
                    }))
                };

                self.i = self.i + 8 + (chunk_size as usize);
                self.i = self.i.next_multiple_of(2);

                chunk
            } else {
                Some(Err(RiffError::TruncatedChunkData))
            }
        } else {
            None
        }
    }
}
