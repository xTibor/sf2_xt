use zerocopy::{LittleEndian as LE, U16, U32, FromBytes, AsBytes, Unaligned};


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
