use std::io::Cursor;

use byteorder::ByteOrder;
use rand_mt::Mt64;

use crate::ec2b::Ec2b;

const SIZE: usize = 4096;
pub struct MhyXorpad([u8; SIZE]);

#[derive(PartialEq)]
pub enum XorpadGenerationMethod {
    Normal,
    ReseedWithSkip,
}

impl MhyXorpad {
    pub fn from_ec2b(ec2b: &[u8]) -> Result<Self, crate::ec2b::DecodeError> {
        let mut r = Cursor::new(ec2b);
        let ec2b = Ec2b::read(&mut r)?;
        Ok(Self::new::<byteorder::LE>(
            ec2b.derive_seed(),
            XorpadGenerationMethod::Normal,
        ))
    }

    #[must_use]
    pub fn new<E: ByteOrder>(seed: u64, method: XorpadGenerationMethod) -> Self {
        let mut mt = Mt64::new(seed);

        if method == XorpadGenerationMethod::ReseedWithSkip {
            let seed = mt.next_u64();
            mt.reseed(seed);
            mt.next_u64();
        }

        let mut buf = [0u8; 4096];

        (0..(SIZE >> 3)).for_each(|i| E::write_u64(&mut buf[i * 8..(i + 1) * 8], mt.next_u64()));
        Self(buf)
    }

    pub fn xor(&self, buf: &mut [u8]) {
        buf.iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v ^= self.0[i % SIZE]);
    }
}
