use zerocopy::{FromBytes, FromZeroes, Unaligned, LE, U16};

use crate::utils::str_from_fixedstr;
use crate::Sf2Result;

#[derive(Debug, FromZeroes, FromBytes, Unaligned)]
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
