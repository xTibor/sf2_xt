use zerocopy::{FromBytes, FromZeroes, Unaligned};

#[derive(Debug, FromZeroes, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2InstrumentModulator {
    // SFModulator sfModSrcOper;
    // SFGenerator sfModDestOper;
    // SHORT modAmount;
    // SFModulator sfModAmtSrcOper;
    // SFTransform sfModTransOper;
}
