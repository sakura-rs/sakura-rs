use byteorder::ReadBytesExt;
use std::io::{Read, Seek};

use crate::FromBinary;

pub struct ExistFlag(Vec<u8>);

impl ExistFlag {
    pub fn exists(&self, index: usize) -> bool {
        let byte_idx = index / 8;

        self.0.len() > byte_idx && (self.0[byte_idx] & (1 << (index % 8))) != 0
    }
}

impl FromBinary for ExistFlag {
    fn from_binary<R: Read + Seek>(r: &mut R) -> std::io::Result<Self> {
        let size = r.read_u8()? as usize;

        let mut data = vec![0u8; size];
        r.read_exact(&mut data)?;
        Ok(Self(data))
    }
}
