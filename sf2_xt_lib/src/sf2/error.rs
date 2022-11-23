use std::error::Error;
use std::fmt;

use crate::riff::RiffError;

#[derive(Debug)]
pub enum Sf2Error {
    RiffError(RiffError),
    InvalidRootChunk,
    MissingChunk(&'static str),
    MalformedZstr,
    MalformedFixedstr,
    MalformedVersionChunk,
}

impl Error for Sf2Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Sf2Error::RiffError(err) => Some(err),
            Sf2Error::InvalidRootChunk => None,
            Sf2Error::MissingChunk(_) => None,
            Sf2Error::MalformedZstr => None,
            Sf2Error::MalformedFixedstr => None,
            Sf2Error::MalformedVersionChunk => None,
        }
    }
}

impl fmt::Display for Sf2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sf2Error::RiffError(_) => write!(f, "RIFF error"),
            Sf2Error::InvalidRootChunk => write!(f, "Invalid root chunk"),
            Sf2Error::MissingChunk(chunk_id) => write!(f, "Missing '{}' chunk", chunk_id),
            Sf2Error::MalformedZstr => write!(f, "Malformed zero-terminated string"),
            Sf2Error::MalformedFixedstr => write!(f, "Malformed fixed-length string"),
            Sf2Error::MalformedVersionChunk => write!(f, "Malformed version chunk"),
        }
    }
}

impl From<RiffError> for Sf2Error {
    fn from(err: RiffError) -> Self {
        Sf2Error::RiffError(err)
    }
}
