use std::error::Error;
use std::fmt;

use zerocopy::{FromBytes, LittleEndian as LE, Unaligned, U16, U32};

use crate::riff::{RiffChunk, RiffError};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug)]
pub enum Sf2Error<'a> {
    RiffError(RiffError),
    InvalidRootChunk { found: &'a str, expected: &'a str },
    MissingChunk(&'a str),
}

pub type Sf2Result<'a, T> = Result<T, Sf2Error<'a>>;

impl<'a> Error for Sf2Error<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Sf2Error::RiffError(err) => Some(err),
            Sf2Error::InvalidRootChunk { .. } => None,
            Sf2Error::MissingChunk(_) => None,
        }
    }
}

impl<'a> fmt::Display for Sf2Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sf2Error::RiffError(_) => write!(f, "RIFF error"),
            Sf2Error::InvalidRootChunk { found, expected } => write!(
                f,
                "Invalid root chunk (found: '{}', expected '{}')",
                found, expected
            ),
            Sf2Error::MissingChunk(chunk_id) => write!(f, "Missing '{}' chunk", chunk_id),
        }
    }
}

impl<'a> From<RiffError> for Sf2Error<'a> {
    fn from(err: RiffError) -> Self {
        Sf2Error::RiffError(err)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2PresetHeader {
    pub preset_name: [u8; 20],
    pub preset: U16<LE>,
    pub bank: U16<LE>,
    pub preset_bag_index: U16<LE>,
    pub library: U32<LE>,
    pub genre: U32<LE>,
    pub morphology: U32<LE>,
}

pub struct Sf2PresetHeaderIterator<'a> {
    buffer: &'a [u8],
    i: usize,
}

impl<'a> Sf2PresetHeaderIterator<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, i: 0 }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct Sf2Soundfont<'a> {
    chunk_sfbk: RiffChunk<'a>,
}

impl<'a> Sf2Soundfont<'a> {
    pub fn new(buffer: &[u8]) -> Sf2Result<Sf2Soundfont> {
        let chunk_sfbk = RiffChunk::new(buffer)?;

        if chunk_sfbk.chunk_id() != "sfbk" {
            return Err(Sf2Error::InvalidRootChunk {
                found: chunk_sfbk.chunk_id(),
                expected: "sfbk",
            });
        }

        Ok(Sf2Soundfont { chunk_sfbk })
    }

    pub fn preset_headers(&self) -> Sf2Result<Sf2PresetHeaderIterator> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk("pdta"))?;

        let chunk_phdr = chunk_pdta
            .subchunk("phdr")?
            .ok_or(Sf2Error::MissingChunk("phdr"))?;

        Ok(Sf2PresetHeaderIterator::new(chunk_phdr.chunk_data()?))
    }
}
