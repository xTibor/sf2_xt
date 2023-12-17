use zerocopy::{FromBytes, FromZeroes, Unaligned};

#[derive(Debug, FromZeroes, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2InstrumentGenerator {
    // SFGenerator sfGenOper;
    // genAmountType genAmount;
}
