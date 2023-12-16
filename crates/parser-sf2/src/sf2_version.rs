use zerocopy::{FromBytes, FromZeroes, Unaligned, LE, U16};

#[derive(Debug, FromZeroes, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2Version {
    pub major: U16<LE>,
    pub minor: U16<LE>,
}
