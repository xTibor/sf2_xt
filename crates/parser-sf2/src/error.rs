use parser_riff::RiffError;

use derive_more::{Display, Error, From};

#[rustfmt::skip]
#[derive(Debug, Display, Error, From)]
pub enum Sf2Error {
    #[display(fmt = "Invalid root chunk")]
    InvalidRootChunk,

    #[display(fmt = "Missing '{chunk_id:}' chunk")]
    MissingChunk {
        chunk_id: &'static str,
    },

    #[display(fmt = "Malformed '{chunk_id:}' chunk")]
    MalformedChunk {
        chunk_id: &'static str,
    },

    #[display(fmt = "Missing terminator record for '{chunk_id:}' chunk")]
    MissingTerminatorRecord {
        chunk_id: &'static str,
    },

    #[display(fmt = "Malformed zero-terminated string")]
    MalformedZstr,

    #[display(fmt = "Malformed fixed-length string")]
    MalformedFixedstr,

    #[display(fmt = "Malformed version chunk")]
    MalformedVersionChunk,

    #[from]
    RiffError(RiffError),
}
