use std::marker::PhantomData;
use std::mem;
use std::slice::ChunksExact;

use zerocopy::FromBytes;

pub trait IsTerminalRecord {
    fn is_terminal_record(&self) -> bool;
}

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
