use zerocopy::FromBytes;

use parser_riff::RiffChunk;

use crate::{
    Sf2Error, Sf2Info, Sf2InstrumentHeader, Sf2InstrumentZone, Sf2PresetHeader, Sf2PresetZone,
    Sf2Result, Sf2SampleHeader,
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

    pub fn preset_headers(&'a self) -> Sf2Result<&'a [Sf2PresetHeader]> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "pdta" })?;

        let chunk_phdr = chunk_pdta
            .subchunk("phdr")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "phdr" })?;

        let (_, slice_phdr) = Sf2PresetHeader::slice_from(chunk_phdr.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "phdr" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "phdr" })?;

        Ok(slice_phdr)
    }

    pub fn preset_zones(&'a self) -> Sf2Result<&'a [Sf2PresetZone]> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "pdta" })?;

        let chunk_pbag = chunk_pdta
            .subchunk("pbag")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "pbag" })?;

        let (_, slice_pbag) = Sf2PresetZone::slice_from(chunk_pbag.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "pbag" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "pbag" })?;

        Ok(slice_pbag)
    }

    pub fn instrument_headers(&'a self) -> Sf2Result<&'a [Sf2InstrumentHeader]> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "pdta" })?;

        let chunk_inst = chunk_pdta
            .subchunk("inst")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "inst" })?;

        let (_, slice_inst) = Sf2InstrumentHeader::slice_from(chunk_inst.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "inst" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "inst" })?;

        Ok(slice_inst)
    }

    pub fn instrument_zones(&'a self) -> Sf2Result<&'a [Sf2InstrumentZone]> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "pdta" })?;

        let chunk_ibag = chunk_pdta
            .subchunk("ibag")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "ibag" })?;

        let (_, slice_ibag) = Sf2InstrumentZone::slice_from(chunk_ibag.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "ibag" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "ibag" })?;

        Ok(slice_ibag)
    }

    pub fn sample_headers(&self) -> Sf2Result<&'a [Sf2SampleHeader]> {
        let chunk_pdta = self
            .chunk_sfbk
            .subchunk("pdta")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "pdta" })?;

        let chunk_shdr = chunk_pdta
            .subchunk("shdr")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "shdr" })?;

        let (_, slice_shdr) = Sf2SampleHeader::slice_from(chunk_shdr.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "shdr" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "shdr" })?;

        Ok(slice_shdr)
    }

    pub fn info(&self) -> Sf2Result<Sf2Info> {
        let chunk_info = self
            .chunk_sfbk
            .subchunk("INFO")?
            .ok_or(Sf2Error::MissingChunk { chunk_id: "INFO" })?;

        Sf2Info::new(chunk_info)
    }
}
