use crate::riff::RiffChunk;

use crate::sf2::{
    Sf2Error, Sf2Info, Sf2Instrument, Sf2PresetHeader, Sf2RecordIterator, Sf2Result, Sf2Sample,
};

pub struct Sf2SoundFont<'a> {
    chunk_sfbk: RiffChunk<'a>,
}

impl<'a> Sf2SoundFont<'a> {
    pub fn new(buffer: &'a [u8]) -> Sf2Result<Sf2SoundFont<'a>> {
        let chunk_sfbk = RiffChunk::new(buffer)?;

        if chunk_sfbk.chunk_id() != "sfbk" {
            return Err(Sf2Error::InvalidRootChunk);
        }

        Ok(Sf2SoundFont { chunk_sfbk })
    }

    // TODO: pub fn preset_headers(&self) -> Sf2Result<&'a [Sf2PresetHeader]>
    pub fn preset_headers(&self) -> Sf2Result<Sf2RecordIterator<Sf2PresetHeader>> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk("pdta"))?;

        let chunk_phdr = chunk_pdta
            .subchunk("phdr")?
            .ok_or(Sf2Error::MissingChunk("phdr"))?;

        Ok(Sf2RecordIterator::new(chunk_phdr.chunk_data()?))
    }

    // TODO: pub fn instruments(&self) -> Sf2Result<&'a [Sf2Instrument]>
    pub fn instruments(&self) -> Sf2Result<Sf2RecordIterator<Sf2Instrument>> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk("pdta"))?;

        let chunk_inst = chunk_pdta
            .subchunk("inst")?
            .ok_or(Sf2Error::MissingChunk("inst"))?;

        Ok(Sf2RecordIterator::new(chunk_inst.chunk_data()?))
    }

    // TODO: pub fn samples(&self) -> Sf2Result<&'a [Sf2Sample]>
    pub fn samples(&self) -> Sf2Result<Sf2RecordIterator<Sf2Sample>> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk("pdta"))?;

        let chunk_shdr = chunk_pdta
            .subchunk("shdr")?
            .ok_or(Sf2Error::MissingChunk("shdr"))?;

        Ok(Sf2RecordIterator::new(chunk_shdr.chunk_data()?))
    }

    pub fn info(&self) -> Sf2Result<Sf2Info> {
        let chunk_info = self
            .chunk_sfbk
            .subchunk("INFO")?
            .ok_or(Sf2Error::MissingChunk("INFO"))?;

        Sf2Info::new(chunk_info)
    }
}
