use zerocopy::FromBytes;

use parser_riff::RiffChunk;

use crate::utils::str_from_zstr;
use crate::{Sf2Error, Sf2Result, Sf2Version};

pub struct Sf2Info<'a> {
    chunk_info: &'a RiffChunk<'a>,
}

impl<'a> Sf2Info<'a> {
    pub fn new(chunk_info: &'a RiffChunk<'a>) -> Sf2Result<Sf2Info<'a>> {
        Ok(Sf2Info { chunk_info })
    }

    fn read_zstr_chunk_opt(&self, chunk_id: &'static str) -> Sf2Result<Option<&'a str>> {
        if let Some(chunk) = self.chunk_info.subchunk_opt(chunk_id)? {
            Ok(Some(str_from_zstr(chunk.chunk_data()?)?))
        } else {
            Ok(None)
        }
    }

    fn read_ver_chunk_opt(&self, chunk_id: &'static str) -> Sf2Result<Option<(u16, u16)>> {
        if let Some(chunk) = self.chunk_info.subchunk_opt(chunk_id)? {
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
            .ok_or(Sf2Error::MissingChunk { chunk_id })?
    }

    fn read_ver_chunk(&self, chunk_id: &'static str) -> Sf2Result<(u16, u16)> {
        self.read_ver_chunk_opt(chunk_id)
            .transpose()
            .ok_or(Sf2Error::MissingChunk { chunk_id })?
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
            .map(|opt| opt.map(|s| s.split(':').filter(|s| !s.is_empty()).collect::<Vec<_>>()))
    }
}
