use zerocopy::FromBytes;

use parser_riff::RiffChunk;

use crate::{
    Sf2Error, Sf2Info, Sf2InstrumentHeader, Sf2InstrumentZone, Sf2PresetHeader, Sf2PresetZone,
    Sf2Result, Sf2SampleHeader,
};

pub struct Sf2SoundFont<'a> {
    chunk_sfbk: RiffChunk<'a>,
}

fn sf2_array_ref<'a, T: FromBytes>(chunk: &RiffChunk<'a>, chunk_id: &str) -> Sf2Result<&'a [T]> {
    let (_, typed_slice) = T::slice_from(chunk.chunk_data()?)
        .ok_or(Sf2Error::MalformedChunk {
            chunk_id: chunk_id.to_owned(),
        })?
        .split_last()
        .ok_or(Sf2Error::MissingTerminatorRecord {
            chunk_id: chunk_id.to_owned(),
        })?;

    Ok(typed_slice)
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

        sf2_array_ref(chunk_phdr, "phdr")
    }

    pub fn preset_zones(&'a self) -> Sf2Result<&'a [Sf2PresetZone]> {
        #[rustfmt::skip]
        let chunk_pbag = self
            .chunk_sfbk
            .subchunk("pdta")?
            .subchunk("pbag")?;

        sf2_array_ref(chunk_pbag, "pbag")
    }

    pub fn instrument_headers(&'a self) -> Sf2Result<&'a [Sf2InstrumentHeader]> {
        #[rustfmt::skip]
        let chunk_inst = self
            .chunk_sfbk
            .subchunk("pdta")?
            .subchunk("inst")?;

        sf2_array_ref(chunk_inst, "inst")
    }

    pub fn instrument_zones(&'a self) -> Sf2Result<&'a [Sf2InstrumentZone]> {
        #[rustfmt::skip]
        let chunk_ibag = self
            .chunk_sfbk
            .subchunk("pdta")?
            .subchunk("ibag")?;

        sf2_array_ref(chunk_ibag, "ibag")
    }

    pub fn sample_headers(&self) -> Sf2Result<&'a [Sf2SampleHeader]> {
        #[rustfmt::skip]
        let chunk_shdr = self
            .chunk_sfbk
            .subchunk("pdta")?
            .subchunk("shdr")?;

        sf2_array_ref(chunk_shdr, "shdr")
    }

    pub fn info(&self) -> Sf2Result<Sf2Info> {
        #[rustfmt::skip]
        let chunk_info = self
            .chunk_sfbk
            .subchunk("INFO")?;

        Sf2Info::new(chunk_info)
    }
}
