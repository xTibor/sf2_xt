use std::error::Error;
use std::ffi::CStr;
use std::slice::ChunksExact;
use std::{fmt, mem};

use zerocopy::{FromBytes, LittleEndian as LE, Unaligned, U16, U32};

use crate::riff::{RiffChunk, RiffError};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug)]
pub enum Sf2Error {
    RiffError(RiffError),
    InvalidRootChunk,
    MissingChunk(&'static str),
    MalformedPresetName,
}

pub type Sf2Result<T> = Result<T, Sf2Error>;

impl Error for Sf2Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Sf2Error::RiffError(err) => Some(err),
            Sf2Error::InvalidRootChunk => None,
            Sf2Error::MissingChunk(_) => None,
            Sf2Error::MalformedPresetName => None,
        }
    }
}

impl fmt::Display for Sf2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sf2Error::RiffError(_) => write!(f, "RIFF error"),
            Sf2Error::InvalidRootChunk => write!(f, "Invalid root chunk"),
            Sf2Error::MissingChunk(chunk_id) => write!(f, "Missing '{}' chunk", chunk_id),
            Sf2Error::MalformedPresetName => write!(f, "Malformed preset name"),
        }
    }
}

impl From<RiffError> for Sf2Error {
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

impl Sf2PresetHeader {
    pub fn preset_name(&self) -> Sf2Result<&str> {
        // preset_name may contain garbage after the zero-terminator (GeneralUser GS)
        CStr::from_bytes_until_nul(&self.preset_name)
            .map_err(|_| Sf2Error::MalformedPresetName)?
            .to_str()
            .map_err(|_| Sf2Error::MalformedPresetName)
    }

    pub fn bank_preset(&self) -> (u16, u16) {
        (self.bank.get(), self.preset.get())
    }

    pub fn bank(&self) -> u16 {
        self.bank.get()
    }

    pub fn preset(&self) -> u16 {
        self.preset.get()
    }
}

pub struct Sf2PresetHeaderIterator<'a> {
    iter: ChunksExact<'a, u8>,
}

impl<'a> Sf2PresetHeaderIterator<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        let iter = buffer.chunks_exact(mem::size_of::<Sf2PresetHeader>());
        Self { iter }
    }
}

impl<'a> Iterator for Sf2PresetHeaderIterator<'a> {
    type Item = Sf2PresetHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(preset_header_raw) = self.iter.next() {
            let preset_header = Sf2PresetHeader::read_from_prefix(preset_header_raw).unwrap();

            if preset_header.preset_name.starts_with(b"EOP\0") {
                None
            } else {
                Some(preset_header)
            }
        } else {
            None
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct Sf2Soundfont<'a> {
    chunk_sfbk: RiffChunk<'a>,
}

impl<'a> Sf2Soundfont<'a> {
    pub fn new(buffer: &'a [u8]) -> Sf2Result<Sf2Soundfont<'a>> {
        let chunk_sfbk = RiffChunk::new(buffer)?;

        if chunk_sfbk.chunk_id() != "sfbk" {
            return Err(Sf2Error::InvalidRootChunk);
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
