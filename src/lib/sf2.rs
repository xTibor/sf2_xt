use zerocopy::{AsBytes, FromBytes, LittleEndian as LE, Unaligned, U16, U32};

#[derive(Debug, FromBytes, AsBytes, Unaligned)]
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
