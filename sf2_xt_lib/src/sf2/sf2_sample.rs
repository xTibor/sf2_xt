use zerocopy::{FromBytes, Unaligned, LE, U16, U32};

use crate::sf2::record_iterator::IsTerminalRecord;
use crate::sf2::utils::str_from_fixedstr;
use crate::sf2::Sf2Result;

#[derive(Debug, FromBytes, Unaligned)]
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

impl IsTerminalRecord for Sf2Sample {
    fn is_terminal_record(&self) -> bool {
        self.sample_name.starts_with(b"EOS\0")
    }
}
