use zerocopy::FromBytes;

use parser_riff::RiffChunk;

use crate::{
    Sf2Error, Sf2Info, Sf2InstrumentGenerator, Sf2InstrumentHeader, Sf2InstrumentModulator,
    Sf2InstrumentZone, Sf2PresetGenerator, Sf2PresetHeader, Sf2PresetModulator, Sf2PresetZone,
    Sf2Result, Sf2SampleHeader,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

trait Sf2TypedSlice {
    fn as_typed_slice<T: FromBytes>(&self) -> Sf2Result<&[T]>;
}

impl<'a> Sf2TypedSlice for RiffChunk<'a> {
    fn as_typed_slice<T: FromBytes>(&self) -> Sf2Result<&[T]> {
        let (_, typed_slice) = T::slice_from(self.chunk_data()?)
            .ok_or(Sf2Error::MalformedChunk {
                chunk_id: self.chunk_id().to_owned(),
            })?
            .split_last()
            .ok_or(Sf2Error::MissingTerminatorRecord {
                chunk_id: self.chunk_id().to_owned(),
            })?;

        Ok(typed_slice)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct Sf2SoundFont<'a> {
    root_chunk: RiffChunk<'a>,
}

impl<'a> Sf2SoundFont<'a> {
    pub fn new(buffer: &'a [u8]) -> Sf2Result<Sf2SoundFont<'a>> {
        let root_chunk = RiffChunk::new(buffer)?;

        if root_chunk.chunk_id() != "sfbk" {
            return Err(Sf2Error::InvalidRootChunk);
        }

        Ok(Sf2SoundFont { root_chunk })
    }

    pub fn preset_headers(&'a self) -> Sf2Result<&'a [Sf2PresetHeader]> {
        self.root_chunk
            .subchunk("pdta")?
            .subchunk("phdr")?
            .as_typed_slice()
    }

    pub fn preset_zones(&'a self) -> Sf2Result<&'a [Sf2PresetZone]> {
        self.root_chunk
            .subchunk("pdta")?
            .subchunk("pbag")?
            .as_typed_slice()
    }

    pub fn preset_generators(&'a self) -> Sf2Result<&'a [Sf2PresetGenerator]> {
        self.root_chunk
            .subchunk("pdta")?
            .subchunk("pgen")?
            .as_typed_slice()
    }

    pub fn preset_modulators(&'a self) -> Sf2Result<&'a [Sf2PresetModulator]> {
        self.root_chunk
            .subchunk("pdta")?
            .subchunk("pmod")?
            .as_typed_slice()
    }

    pub fn instrument_headers(&'a self) -> Sf2Result<&'a [Sf2InstrumentHeader]> {
        self.root_chunk
            .subchunk("pdta")?
            .subchunk("inst")?
            .as_typed_slice()
    }

    pub fn instrument_zones(&'a self) -> Sf2Result<&'a [Sf2InstrumentZone]> {
        self.root_chunk
            .subchunk("pdta")?
            .subchunk("ibag")?
            .as_typed_slice()
    }

    pub fn instrument_generators(&'a self) -> Sf2Result<&'a [Sf2InstrumentGenerator]> {
        self.root_chunk
            .subchunk("pdta")?
            .subchunk("igen")?
            .as_typed_slice()
    }

    pub fn instrument_modulators(&'a self) -> Sf2Result<&'a [Sf2InstrumentModulator]> {
        self.root_chunk
            .subchunk("pdta")?
            .subchunk("imod")?
            .as_typed_slice()
    }

    pub fn sample_headers(&'a self) -> Sf2Result<&'a [Sf2SampleHeader]> {
        self.root_chunk
            .subchunk("pdta")?
            .subchunk("shdr")?
            .as_typed_slice()
    }

    pub fn info(&self) -> Sf2Result<Sf2Info> {
        #[rustfmt::skip]
        let chunk_info = self
            .root_chunk
            .subchunk("INFO")?;

        Sf2Info::new(chunk_info)
    }
}
