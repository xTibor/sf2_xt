mod error;
mod raw_chunk;
mod riff_chunk;

pub use error::RiffError;
pub use raw_chunk::{RawChunk, RawChunkIterator};
pub use riff_chunk::RiffChunk;

pub type RiffResult<T> = Result<T, RiffError>;
