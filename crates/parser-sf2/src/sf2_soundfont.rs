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
        #[rustfmt::skip]
        let chunk_phdr = self
            .chunk_sfbk
            .subchunk("pdta")?
            .subchunk("phdr")?;

        let (_, slice_phdr) = Sf2PresetHeader::slice_from(chunk_phdr.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "phdr" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "phdr" })?;

        Ok(slice_phdr)
    }

    pub fn preset_zones(&'a self) -> Sf2Result<&'a [Sf2PresetZone]> {
        #[rustfmt::skip]
        let chunk_pbag = self
            .chunk_sfbk
            .subchunk("pdta")?
            .subchunk("pbag")?;

        let (_, slice_pbag) = Sf2PresetZone::slice_from(chunk_pbag.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "pbag" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "pbag" })?;

        Ok(slice_pbag)
    }

    pub fn instrument_headers(&'a self) -> Sf2Result<&'a [Sf2InstrumentHeader]> {
        #[rustfmt::skip]
        let chunk_inst = self
            .chunk_sfbk
            .subchunk("pdta")?
            .subchunk("inst")?;

        let (_, slice_inst) = Sf2InstrumentHeader::slice_from(chunk_inst.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "inst" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "inst" })?;

        Ok(slice_inst)
    }

    pub fn instrument_zones(&'a self) -> Sf2Result<&'a [Sf2InstrumentZone]> {
        #[rustfmt::skip]
        let chunk_ibag = self
            .chunk_sfbk
            .subchunk("pdta")?
            .subchunk("ibag")?;

        let (_, slice_ibag) = Sf2InstrumentZone::slice_from(chunk_ibag.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "ibag" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "ibag" })?;

        Ok(slice_ibag)
    }

    pub fn sample_headers(&self) -> Sf2Result<&'a [Sf2SampleHeader]> {
        #[rustfmt::skip]
        let chunk_shdr = self
            .chunk_sfbk
            .subchunk("pdta")?
            .subchunk("shdr")?;

        let (_, slice_shdr) = Sf2SampleHeader::slice_from(chunk_shdr.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk { chunk_id: "shdr" })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord { chunk_id: "shdr" })?;

        Ok(slice_shdr)
    }

    pub fn info(&self) -> Sf2Result<Sf2Info> {
        #[rustfmt::skip]
        let chunk_info = self
            .chunk_sfbk
            .subchunk("INFO")?;

        Sf2Info::new(chunk_info)
    }
}
