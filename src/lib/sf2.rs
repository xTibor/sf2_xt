use std::error::Error;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::slice::ChunksExact;
use std::{fmt, mem, str};

use itertools::Itertools;
use zerocopy::{FromBytes, LittleEndian as LE, Unaligned, U16, U32};

use crate::riff::{RiffChunk, RiffError};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug)]
pub enum Sf2Error {
    RiffError(RiffError),
    InvalidRootChunk,
    MissingChunk(&'static str),
    MalformedZstr,
    MalformedFixedstr,
    MalformedVersionChunk,
}

pub type Sf2Result<T> = Result<T, Sf2Error>;

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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

fn str_from_zstr<'a>(data: &'a [u8]) -> Sf2Result<&'a str> {
    Ok(CStr::from_bytes_until_nul(data)
        .map_err(|_| Sf2Error::MalformedZstr)?
        .to_str()
        .map_err(|_| Sf2Error::MalformedZstr)?)
}

fn str_from_fixedstr<'a>(data: &'a [u8]) -> Sf2Result<&'a str> {
    // Fixed-length strings may contain garbage after the zero-terminator that may
    // cause issues with the string conversion. (GeneralUser GS)
    let terminator_pos = data.iter().position(|&b| b == b'\0').unwrap_or(data.len());

    str::from_utf8(&data[..terminator_pos]).map_err(|_| Sf2Error::MalformedFixedstr)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2Version {
    pub major: U16<LE>,
    pub minor: U16<LE>,
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
        str_from_fixedstr(&self.preset_name)
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
            let preset_header = Sf2PresetHeader::read_from(preset_header_raw).unwrap();

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

pub struct Sf2Info<'a> {
    chunk_info: &'a RiffChunk<'a>,
}

impl<'a> Sf2Info<'a> {
    pub fn new(chunk_info: &'a RiffChunk<'a>) -> Sf2Result<Sf2Info<'a>> {
        Ok(Sf2Info { chunk_info })
    }

    fn read_zstr_chunk_opt(&self, chunk_id: &'static str) -> Sf2Result<Option<&'a str>> {
        if let Some(chunk) = self.chunk_info.subchunk(chunk_id)? {
            Ok(Some(str_from_zstr(chunk.chunk_data()?)?))
        } else {
            Ok(None)
        }
    }

    fn read_ver_chunk_opt(&self, chunk_id: &'static str) -> Sf2Result<Option<(u16, u16)>> {
        if let Some(chunk) = self.chunk_info.subchunk(chunk_id)? {
            if let Some(Sf2Version { major, minor }) = Sf2Version::read_from(chunk.chunk_data()?) {
                Ok(Some((major.get(), minor.get())))
            } else {
                Err(Sf2Error::MalformedVersionChunk)
            }
        } else {
            Ok(None)
        }
    }

    fn read_zstr_chunk(&self, chunk_id: &'static str) -> Sf2Result<&'a str> {
        self.read_zstr_chunk_opt(chunk_id)
            .transpose()
            .ok_or(Sf2Error::MissingChunk(chunk_id))?
    }

    fn read_ver_chunk(&self, chunk_id: &'static str) -> Sf2Result<(u16, u16)> {
        self.read_ver_chunk_opt(chunk_id)
            .transpose()
            .ok_or(Sf2Error::MissingChunk(chunk_id))?
    }

    pub fn format_version(&self) -> Sf2Result<(u16, u16)> {
        self.read_ver_chunk("ifil")
    }

    pub fn sound_engine(&self) -> Sf2Result<&'a str> {
        self.read_zstr_chunk("isng")
    }

    pub fn soundfont_name(&self) -> Sf2Result<&'a str> {
        self.read_zstr_chunk("INAM")
    }

    pub fn rom_name(&self) -> Sf2Result<Option<&'a str>> {
        self.read_zstr_chunk_opt("irom")
    }

    pub fn rom_version(&self) -> Sf2Result<Option<(u16, u16)>> {
        self.read_ver_chunk_opt("iver")
    }

    pub fn date(&self) -> Sf2Result<Option<&'a str>> {
        self.read_zstr_chunk_opt("ICRD")
    }

    pub fn author(&self) -> Sf2Result<Option<&'a str>> {
        self.read_zstr_chunk_opt("IENG")
    }

    pub fn product(&self) -> Sf2Result<Option<&'a str>> {
        self.read_zstr_chunk_opt("IPRD")
    }

    pub fn copyright(&self) -> Sf2Result<Option<&'a str>> {
        self.read_zstr_chunk_opt("ICOP")
    }

    pub fn comment(&self) -> Sf2Result<Option<&'a str>> {
        self.read_zstr_chunk_opt("ICMT")
    }

    pub fn soundfont_tools(&self) -> Sf2Result<Option<Vec<&'a str>>> {
        self.read_zstr_chunk_opt("ISFT")
            .map(|opt| opt.map(|s| s.split(':').filter(|s| !s.is_empty()).collect_vec()))
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2Instrument {
    pub instrument_name: [u8; 20],
    pub instrument_bag_index: U16<LE>,
}

impl Sf2Instrument {
    pub fn instrument_name(&self) -> Sf2Result<&str> {
        str_from_fixedstr(&self.instrument_name)
    }
}

pub struct Sf2InstrumentIterator<'a> {
    iter: ChunksExact<'a, u8>,
}

impl<'a> Sf2InstrumentIterator<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        let iter = buffer.chunks_exact(mem::size_of::<Sf2Instrument>());
        Self { iter }
    }
}

impl<'a> Iterator for Sf2InstrumentIterator<'a> {
    type Item = Sf2Instrument;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(instrument_raw) = self.iter.next() {
            let instrument = Sf2Instrument::read_from(instrument_raw).unwrap();

            if instrument.instrument_name.starts_with(b"EOI\0") {
                None
            } else {
                Some(instrument)
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

    pub fn instruments(&self) -> Sf2Result<Sf2InstrumentIterator> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk("pdta"))?;

        let chunk_inst = chunk_pdta
            .subchunk("inst")?
            .ok_or(Sf2Error::MissingChunk("inst"))?;

        Ok(Sf2InstrumentIterator::new(chunk_inst.chunk_data()?))
    }

    pub fn info(&self) -> Sf2Result<Sf2Info> {
        let chunk_info = self
            .chunk_sfbk
            .subchunk("INFO")?
            .ok_or(Sf2Error::MissingChunk("INFO"))?;

        Sf2Info::new(chunk_info)
    }
}
