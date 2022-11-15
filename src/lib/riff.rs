use std::fmt::{self, Debug};
use std::str;

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

impl<'a> RawChunk<'a> {
    pub fn subchunks(&self) -> RawChunkIterator<'a> {
        match self {
            RawChunk::Container { chunk_data, .. } => RawChunkIterator::new(chunk_data),
            RawChunk::Normal { .. } => panic!("Normal chunks cannot have subchunks"),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

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
    type Item = RawChunk<'a>;

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

                    Some(RawChunk::Container {
                        chunk_type,
                        chunk_id,
                        chunk_data,
                    })
                } else {
                    let chunk_data = &self.buffer[self.i + 8..self.i + 8 + (chunk_size as usize)];

                    Some(RawChunk::Normal {
                        chunk_id,
                        chunk_data,
                    })
                };

                self.i = self.i + 8 + (chunk_size as usize);
                self.i = self.i.next_multiple_of(2);

                chunk
            } else {
                None
            }
        } else {
            None
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub enum RiffChunk<'a> {
    Container {
        chunk_type: &'a str,
        chunk_id: &'a str,
        subchunks: Vec<RiffChunk<'a>>,
    },
    Normal {
        chunk_id: &'a str,
        chunk_data: &'a [u8],
    },
}

impl<'a> Debug for RiffChunk<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiffChunk::Container {
                chunk_type,
                chunk_id,
                subchunks,
            } => f
                .debug_struct("RiffChunk::Container")
                .field("chunk_type", chunk_type)
                .field("chunk_id", chunk_id)
                .field("subchunks", subchunks)
                .finish(),
            RiffChunk::Normal { chunk_id, .. } => f
                .debug_struct("RiffChunk::Normal")
                .field("chunk_id", chunk_id)
                .field("chunk_data", &"...")
                .finish(),
        }
    }
}

impl<'a> From<RawChunk<'a>> for RiffChunk<'a> {
    fn from(raw_chunk: RawChunk<'a>) -> Self {
        match raw_chunk {
            RawChunk::Container {
                chunk_type,
                chunk_id,
                chunk_data,
            } => {
                let subchunks = RawChunkIterator::new(chunk_data)
                    .map(RiffChunk::from)
                    .collect::<Vec<_>>();

                RiffChunk::Container {
                    chunk_type: str::from_utf8(chunk_type).unwrap(),
                    chunk_id: str::from_utf8(chunk_id).unwrap(),
                    subchunks,
                }
            }
            RawChunk::Normal {
                chunk_id,
                chunk_data,
            } => RiffChunk::Normal {
                chunk_id: str::from_utf8(chunk_id).unwrap(),
                chunk_data,
            },
        }
    }
}

impl<'a> RiffChunk<'a> {
    pub fn new(buffer: &[u8]) -> RiffChunk {
        RawChunkIterator::new(buffer).next().unwrap().into()
    }

    pub fn chunk_id(&self) -> &'a str {
        match self {
            RiffChunk::Container { chunk_id, .. } => chunk_id,
            RiffChunk::Normal { chunk_id, .. } => chunk_id,
        }
    }

    pub fn chunk_data(&self) -> &'a [u8] {
        match self {
            RiffChunk::Container { .. } => panic!("Container chunks have no data"),
            RiffChunk::Normal { chunk_data, .. } => chunk_data,
        }
    }

    pub fn subchunk(&self, chunk_id: &str) -> Option<&RiffChunk<'a>> {
        match self {
            RiffChunk::Container { subchunks, .. } => subchunks
                .iter()
                .find(|subchunk| subchunk.chunk_id() == chunk_id),
            RiffChunk::Normal { .. } => None,
        }
    }

    pub fn is_container(&self) -> bool {
        matches!(self, RiffChunk::Container { .. })
    }
}
