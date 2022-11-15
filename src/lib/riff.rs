use std::error::Error;
use std::fmt::{self, Debug};
use std::str::{self, Utf8Error};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug)]
pub enum RiffError {
    MissingChunk,
    NormalChunkNoSubchunks,
    ContainerChunkNoData,
    TruncatedChunkData,
    MalformedChunkId(Utf8Error),
    MalformedChunkType(Utf8Error),
}

pub type RiffResult<T> = Result<T, RiffError>;

impl Error for RiffError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RiffError::MissingChunk => None,
            RiffError::NormalChunkNoSubchunks => None,
            RiffError::ContainerChunkNoData => None,
            RiffError::TruncatedChunkData => None,
            RiffError::MalformedChunkId(ref err) => Some(err),
            RiffError::MalformedChunkType(ref err) => Some(err),
        }
    }
}

impl fmt::Display for RiffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiffError::MissingChunk => write!(f, "Missing chunk"),
            RiffError::NormalChunkNoSubchunks => write!(f, "Normal chunks cannot have subchunks"),
            RiffError::ContainerChunkNoData => write!(f, "Container chunks cannot have data"),
            RiffError::TruncatedChunkData => write!(f, "Truncated chunk data"),
            RiffError::MalformedChunkId(_) => write!(f, "Malformed chunk ID"),
            RiffError::MalformedChunkType(_) => write!(f, "Malformed chunk type"),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

enum RawChunk<'a> {
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

struct RawChunkIterator<'a> {
    buffer: &'a [u8],
    i: usize,
}

impl<'a> RawChunkIterator<'a> {
    fn new(buffer: &'a [u8]) -> Self {
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

impl<'a> TryFrom<RawChunk<'a>> for RiffChunk<'a> {
    type Error = RiffError;

    fn try_from(raw_chunk: RawChunk<'a>) -> RiffResult<Self> {
        match raw_chunk {
            RawChunk::Container {
                chunk_type,
                chunk_id,
                chunk_data,
            } => {
                let subchunks = RawChunkIterator::new(chunk_data)
                    .map(|raw_chunk| RiffChunk::try_from(raw_chunk?))
                    .collect::<RiffResult<Vec<RiffChunk>>>()?;

                Ok(RiffChunk::Container {
                    chunk_type: str::from_utf8(chunk_type) // TODO: str::from_ascii()
                        .map_err(|err| RiffError::MalformedChunkType(err))?,
                    chunk_id: str::from_utf8(chunk_id) // TODO: str::from_ascii()
                        .map_err(|err| RiffError::MalformedChunkId(err))?,
                    subchunks,
                })
            }
            RawChunk::Normal {
                chunk_id,
                chunk_data,
            } => Ok(RiffChunk::Normal {
                chunk_id: str::from_utf8(chunk_id) // TODO: str::from_ascii()
                    .map_err(|err| RiffError::MalformedChunkId(err))?,
                chunk_data,
            }),
        }
    }
}

impl<'a> RiffChunk<'a> {
    pub fn new(buffer: &[u8]) -> RiffResult<RiffChunk> {
        let raw_chunk = RawChunkIterator::new(buffer)
            .next()
            .transpose()?
            .ok_or(RiffError::MissingChunk)?;
        raw_chunk.try_into()
    }

    pub fn chunk_id(&self) -> &'a str {
        match self {
            RiffChunk::Container { chunk_id, .. } => chunk_id,
            RiffChunk::Normal { chunk_id, .. } => chunk_id,
        }
    }

    pub fn chunk_data(&self) -> RiffResult<&'a [u8]> {
        match self {
            RiffChunk::Container { .. } => Err(RiffError::ContainerChunkNoData),
            RiffChunk::Normal { chunk_data, .. } => Ok(chunk_data),
        }
    }

    pub fn subchunk(&self, chunk_id: &str) -> RiffResult<Option<&RiffChunk<'a>>> {
        match self {
            RiffChunk::Container { subchunks, .. } => Ok(subchunks
                .iter()
                .find(|subchunk| subchunk.chunk_id() == chunk_id)),
            RiffChunk::Normal { .. } => Err(RiffError::NormalChunkNoSubchunks),
        }
    }

    pub fn subchunks(&self, chunk_id: &str) -> RiffResult<Vec<&RiffChunk<'a>>> {
        match self {
            RiffChunk::Container { subchunks, .. } => Ok(subchunks
                .iter()
                .filter(|subchunk| subchunk.chunk_id() == chunk_id)
                .collect::<Vec<&RiffChunk>>()),
            RiffChunk::Normal { .. } => Err(RiffError::NormalChunkNoSubchunks),
        }
    }

    pub fn is_container(&self) -> bool {
        matches!(self, RiffChunk::Container { .. })
    }
}
