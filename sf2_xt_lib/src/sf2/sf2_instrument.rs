use zerocopy::{FromBytes, Unaligned, LE, U16};

use crate::sf2::record_iterator::IsTerminalRecord;
use crate::sf2::utils::str_from_fixedstr;
use crate::sf2::Sf2Result;

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

impl IsTerminalRecord for Sf2Instrument {
    fn is_terminal_record(&self) -> bool {
        self.instrument_name.starts_with(b"EOI\0")
    }
}
