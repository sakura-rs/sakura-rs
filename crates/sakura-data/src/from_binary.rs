use std::io::{Read, Seek};

use byteorder::{ReadBytesExt, LE};
use varint_rs::VarintReader;

pub trait FromBinary: Send + Sync + Sized {
    fn from_binary<R: Read + Seek>(r: &mut R) -> std::io::Result<Self>;
}

impl FromBinary for u32 {
    fn from_binary<R: Read + Seek>(r: &mut R) -> std::io::Result<Self> {
        r.read_u32_varint()
    }
}

impl FromBinary for u64 {
    fn from_binary<R: Read + Seek>(r: &mut R) -> std::io::Result<Self> {
        r.read_u64_varint()
    }
}

impl FromBinary for i32 {
    fn from_binary<R: Read + Seek>(r: &mut R) -> std::io::Result<Self> {
        let unsigned = u32::from_binary(r)?;
        Ok(-((unsigned & 1) as i32) ^ (unsigned >> 1) as i32)
    }
}

impl FromBinary for bool {
    fn from_binary<R: Read + Seek>(r: &mut R) -> std::io::Result<Self> {
        Ok(r.read_u8()? != 0)
    }
}

impl FromBinary for f32 {
    fn from_binary<R: Read + Seek>(r: &mut R) -> std::io::Result<Self> {
        r.read_f32::<LE>()
    }
}

impl FromBinary for String {
    fn from_binary<R: Read + Seek>(r: &mut R) -> std::io::Result<Self> {
        let len = r.read_usize_varint()?;
        let mut buf = vec![0u8; len];
        r.read_exact(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

impl<T> FromBinary for Vec<T>
where
    T: FromBinary,
{
    fn from_binary<R: Read + Seek>(r: &mut R) -> std::io::Result<Self> {
        let count = r.read_usize_varint()?;
        let mut out = Vec::with_capacity(count);

        for _ in 0..count {
            out.push(T::from_binary(r)?);
        }

        Ok(out)
    }
}
