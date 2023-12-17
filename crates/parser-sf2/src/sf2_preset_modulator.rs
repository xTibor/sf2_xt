use zerocopy::{FromBytes, FromZeroes, Unaligned};

#[derive(Debug, FromZeroes, FromBytes, Unaligned)]
#[repr(packed)]
pub struct Sf2PresetModulator {
    // SFModulator sfModSrcOper;
    // SFGenerator sfModDestOper;
    // SHORT modAmount;
    // SFModulator sfModAmtSrcOper;
    // SFTransform sfModTransOper;
}
