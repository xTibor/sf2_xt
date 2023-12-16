use zerocopy::{FromBytes, FromZeroes, Unaligned, LE, U16, U32};

use crate::utils::str_from_fixedstr;
use crate::Sf2Result;

#[derive(Debug, FromZeroes, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2Sample {
    pub sample_name: [u8; 20],
    pub start: U32<LE>,
    pub end: U32<LE>,
    pub start_loop: U32<LE>,
    pub end_loop: U32<LE>,
    pub sample_rate: U32<LE>,
    pub original_pitch: u8,
    pub pitch_correction: i8,
    pub sample_link: U16<LE>,
    pub sample_type: U16<LE>, //TODO: SFSampleLink sfSampleType;
}

impl Sf2Sample {
    pub fn sample_name(&self) -> Sf2Result<&str> {
        str_from_fixedstr(&self.sample_name)
    }
}
