use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RiffError {
    MissingChunk,
    NormalChunkNoSubchunks,
    ContainerChunkNoData,
    TruncatedChunkData,
    MalformedIdentifier,
}

impl Error for RiffError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RiffError::MissingChunk => None,
            RiffError::NormalChunkNoSubchunks => None,
            RiffError::ContainerChunkNoData => None,
            RiffError::TruncatedChunkData => None,
            RiffError::MalformedIdentifier => None,
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
            RiffError::MalformedIdentifier => write!(f, "Malformed identifier"),
        }
    }
}
