use std::marker::PhantomData;
use std::mem;
use std::slice::ChunksExact;

use zerocopy::FromBytes;

pub trait IsTerminalRecord {
    fn is_terminal_record(&self) -> bool;
}

// TODO: Remove `Sf2RecordIterator` when `zerocopy` implements slice casts:
// - https://github.com/google/zerocopy/issues/98
//
// Have to ditch this iterator-based parsing approach in favor of slice casts
// because some SF2 structures depend on subsequent items, for example
// calculating the number of instrument zones/preset zones from the
// wInstBagNdx/wPresetBagNdx fields. There's also the record indexing issue that
// is much cleaner/easier to do with slices than iterators.
pub struct Sf2RecordIterator<'a, T>
where
    T: FromBytes + IsTerminalRecord,
{
    iter: ChunksExact<'a, u8>,
    phantom: PhantomData<T>,
}

impl<'a, T> Sf2RecordIterator<'a, T>
where
    T: FromBytes + IsTerminalRecord,
{
    pub(crate) fn new(buffer: &'a [u8]) -> Self {
        let iter = buffer.chunks_exact(mem::size_of::<T>());
        Self {
            iter,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Sf2RecordIterator<'a, T>
where
    T: FromBytes + IsTerminalRecord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(raw_bytes) = self.iter.next() {
            let record = T::read_from(raw_bytes).unwrap();

            if record.is_terminal_record() {
                None
            } else {
                Some(record)
            }
        } else {
            None
        }
    }
}
