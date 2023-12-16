use zerocopy::{FromBytes, FromZeroes, Unaligned, LE, U16};

#[derive(Debug, FromZeroes, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2PresetZone {
    pub generator_index: U16<LE>,
    pub modulator_index: U16<LE>,
}
