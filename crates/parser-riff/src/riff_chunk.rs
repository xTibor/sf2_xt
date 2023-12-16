use std::fmt::{self, Debug};
use std::str;

use crate::{RawChunk, RawChunkIterator, RiffError, RiffResult};

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
        fn from_fourcc(input: &[u8]) -> RiffResult<&str> {
            let (left, right) = {
                let split_position = input.iter().position(|&b| b == b' ').unwrap_or(input.len());
                input.split_at(split_position)
            };

            if left.iter().all(|&b| b.is_ascii_alphanumeric())
                && right.iter().all(|&b| b == b' ')
                && !left.is_empty()
            {
                Ok(unsafe { str::from_utf8_unchecked(input) })
            } else {
                Err(RiffError::MalformedIdentifier)
            }
        }

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
                    chunk_type: from_fourcc(chunk_type)?,
                    chunk_id: from_fourcc(chunk_id)?,
                    subchunks,
                })
            }
            RawChunk::Normal {
                chunk_id,
                chunk_data,
            } => Ok(RiffChunk::Normal {
                chunk_id: from_fourcc(chunk_id)?,
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

    pub fn subchunks(&self) -> RiffResult<&[RiffChunk<'a>]> {
        match self {
            RiffChunk::Container { subchunks, .. } => Ok(subchunks),
            RiffChunk::Normal { .. } => Err(RiffError::NormalChunkNoSubchunks),
        }
    }

    pub fn is_container(&self) -> bool {
        matches!(self, RiffChunk::Container { .. })
    }
}
