// SPDX-License-Identifier: Apache-2.0

//! Helpful primitives for developing the crate.

// pub mod array;
pub mod cached_chain;
pub(crate) mod hexline;
mod impl_const_id;
pub mod parser_helper;

use std::{
    io::{Read, Result, Write},
    mem::size_of,
    slice::from_raw_parts,
};

use zerocopy::FromBytes;

pub trait TypeLoad: Read {
    fn load<T: FromBytes>(&mut self) -> Result<T> {
        T::read_from_io(self)
    }
}

pub trait TypeSave: Write {
    fn save<T: Sized + Copy>(&mut self, value: &T) -> Result<()> {
        let p = value as *const T as *const u8;
        let s = unsafe { from_raw_parts(p, size_of::<T>()) };
        self.write_all(s)
    }
}

impl<T: Read> TypeLoad for T {}
impl<T: Write> TypeSave for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_reads_value() {
        let mut reader: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
        let value: [u32; 2] = reader.load().unwrap();
        assert_eq!(
            value,
            [
                u32::from_ne_bytes([1, 2, 3, 4]),
                u32::from_ne_bytes([5, 6, 7, 8])
            ]
        );
    }

    #[test]
    fn test_load_short_input() {
        let mut reader: &[u8] = &[1, 2, 3];
        let err = reader.load::<u32>().unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
    }
}
