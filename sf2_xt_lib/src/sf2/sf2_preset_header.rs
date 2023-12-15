use zerocopy::{FromBytes, FromZeroes, Unaligned, LE, U16, U32};

use crate::sf2::record_iterator::IsTerminalRecord;
use crate::sf2::utils::str_from_fixedstr;
use crate::sf2::Sf2Result;

#[derive(Debug, FromZeroes, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2PresetHeader {
    pub preset_name: [u8; 20],
    pub preset: U16<LE>,
    pub bank: U16<LE>,
    pub preset_bag_index: U16<LE>,
    pub library: U32<LE>,
    pub genre: U32<LE>,
    pub morphology: U32<LE>,
}

impl Sf2PresetHeader {
    pub fn preset_name(&self) -> Sf2Result<&str> {
        str_from_fixedstr(&self.preset_name)
    }

    pub fn bank_preset(&self) -> (u16, u16) {
        (self.bank.get(), self.preset.get())
    }

    pub fn bank(&self) -> u16 {
        self.bank.get()
    }

    pub fn preset(&self) -> u16 {
        self.preset.get()
    }
}

impl IsTerminalRecord for Sf2PresetHeader {
    fn is_terminal_record(&self) -> bool {
        self.preset_name.starts_with(b"EOP\0")
    }
}
