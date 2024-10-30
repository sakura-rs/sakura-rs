mod magic;
mod mhy_aes;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use magic::{AES_XORPAD_TABLE, KEY_XORPAD_TABLE};
use mhy_aes::oqs_mhy128_enc_c;
use std::io::Read;
use thiserror::Error;

const HEAD_MAGIC: u32 = 0x45633262; // "Ec2b"
const KEY_SIZE: usize = 16;
const DATA_SIZE: usize = 2048;

pub struct Ec2b {
    key: [u8; KEY_SIZE],
    data: [u8; DATA_SIZE],
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("magic mismatch, expected: {HEAD_MAGIC}, got: {0}")]
    MagicMismatch(u32),
    #[error("invalid key size, expected: {KEY_SIZE}, got: {0}")]
    InvalidKeySize(usize),
    #[error("invalid data size, expected: {DATA_SIZE}, got: {0}")]
    InvalidDataSize(usize),
}

impl Ec2b {
    const XOR_MAGIC: u64 = 0xCEAC3B5A867837AC;

    pub fn read<R: Read>(r: &mut R) -> Result<Self, DecodeError> {
        let magic = r.read_u32::<BigEndian>()?;
        matches!(magic, HEAD_MAGIC)
            .then_some(())
            .ok_or(DecodeError::MagicMismatch(magic))?;

        let key_size = r.read_u32::<LittleEndian>()? as usize;
        matches!(key_size, KEY_SIZE)
            .then_some(())
            .ok_or(DecodeError::InvalidKeySize(key_size))?;

        let mut key = [0u8; KEY_SIZE];
        r.read_exact(&mut key)?;

        let data_size = r.read_u32::<LittleEndian>()? as usize;
        matches!(data_size, DATA_SIZE)
            .then_some(())
            .ok_or(DecodeError::InvalidDataSize(data_size))?;

        let mut data = [0u8; DATA_SIZE];
        r.read_exact(&mut data)?;

        Self::key_scramble(&mut key);
        (0..16).for_each(|i| key[i] ^= KEY_XORPAD_TABLE[i]);

        Ok(Self { key, data })
    }

    #[must_use]
    pub fn derive_seed(&self) -> u64 {
        let val = self
            .data
            .chunks_exact(8)
            .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
            .fold(0xFFFFFFFFFFFFFFFF, |val, i| val ^ i);

        let key_qword_0 = u64::from_le_bytes(self.key[0..8].try_into().unwrap());
        let key_qword_1 = u64::from_le_bytes(self.key[8..16].try_into().unwrap());

        key_qword_1 ^ Self::XOR_MAGIC ^ val ^ key_qword_0
    }

    fn key_scramble(key: &mut [u8]) {
        let mut round_keys = [0u8; 176];
        for round in 0..11 {
            for i in 0..16 {
                for j in 0..16 {
                    let idx = (round << 8) + (i * 16) + j;
                    round_keys[round * 16 + i] ^=
                        AES_XORPAD_TABLE[1][idx] ^ AES_XORPAD_TABLE[0][idx];
                }
            }
        }

        let mut chip = [0u8; 16];
        oqs_mhy128_enc_c(key, &round_keys, &mut chip);

        key.copy_from_slice(&chip);
    }
}
