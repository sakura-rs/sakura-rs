use crate::{PacketHead, Protobuf, ProtobufDecodeError};
use byteorder::{ByteOrder, BE};

pub struct NetPacket<Proto> {
    pub head: PacketHead,
    pub body: Proto,
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("head magic mismatch")]
    HeadMagicMismatch,
    #[error("tail magic mismatch")]
    TailMagicMismatch,
    #[error("input buffer is less than overhead, len: {0}, overhead: {1}")]
    InputLessThanOverhead(usize, usize),
    #[error("out of bounds ({0}/{1})")]
    OutOfBounds(usize, usize),
    #[error("failed to decode PacketHead: {0}")]
    HeadDecode(ProtobufDecodeError),
    #[error("failed to decode body: {0}")]
    BodyDecode(ProtobufDecodeError),
}

#[derive(thiserror::Error, Debug)]
pub enum ProtocolConversionError {
    #[error("failed to decode: {0}")]
    Decode(#[from] ProtobufDecodeError),
    #[error("normal proto for cmd_id: {0} not found")]
    NotFound(u16),
    #[error("normal proto for arg_type: {0:?} not found")]
    NotFoundCombatArgument(crate::normal::CombatTypeArgument),
}

const OVERHEAD: usize = 12;
const HEAD_MAGIC: [u8; 2] = 0x4567_u16.to_be_bytes();
const TAIL_MAGIC: [u8; 2] = 0x89AB_u16.to_be_bytes();

impl<Proto> NetPacket<Proto>
where
    Proto: crate::YSMessage + Default,
{
    pub fn new(proto: Proto) -> Self {
        Self {
            body: proto,
            head: PacketHead::default(),
        }
    }

    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        if buf.len() < OVERHEAD {
            return Err(DecodeError::InputLessThanOverhead(buf.len(), OVERHEAD));
        }

        if &buf[0..2] != HEAD_MAGIC {
            return Err(DecodeError::HeadMagicMismatch);
        }

        let head_len = BE::read_u16(&buf[4..6]) as usize;
        let body_len = BE::read_u32(&buf[6..10]) as usize;

        let required_len = 2 + head_len + body_len;
        if required_len > buf.len() {
            return Err(DecodeError::OutOfBounds(required_len, buf.len()));
        }

        if &buf[(10 + head_len + body_len)..(12 + head_len + body_len)] != TAIL_MAGIC {
            return Err(DecodeError::TailMagicMismatch);
        }

        let head = PacketHead::decode(&buf[10..10 + head_len]).map_err(DecodeError::HeadDecode)?;
        let body = Proto::decode(&buf[10 + head_len..10 + head_len + body_len])
            .map_err(DecodeError::BodyDecode)?;

        Ok(NetPacket { head, body })
    }

    pub fn encode(&self) -> Box<[u8]> {
        let head_len = self.head.encoded_len();
        let body_len = self.body.encoded_len();
        let encoded_len = OVERHEAD + head_len + body_len;

        let mut buf = vec![0u8; encoded_len];
        (&mut buf[0..2]).copy_from_slice(&HEAD_MAGIC);
        BE::write_u16(&mut buf[2..4], self.body.get_cmd_id());
        BE::write_u16(&mut buf[4..6], head_len as u16);
        BE::write_u32(&mut buf[6..10], body_len as u32);

        self.head
            .encode(&mut buf[10..10 + head_len].as_mut())
            .unwrap();

        self.body
            .encode(&mut buf[10 + head_len..10 + head_len + body_len].as_mut())
            .unwrap();

        (&mut buf[10 + head_len + body_len..12 + head_len + body_len]).copy_from_slice(&TAIL_MAGIC);
        buf.into_boxed_slice()
    }
}

pub fn read_cmd_id(buf: &[u8]) -> Result<u16, DecodeError> {
    if buf.len() < OVERHEAD {
        return Err(DecodeError::InputLessThanOverhead(buf.len(), OVERHEAD));
    }

    (&buf[0..2] == HEAD_MAGIC)
        .then_some(BE::read_u16(&buf[2..4]))
        .ok_or(DecodeError::HeadMagicMismatch)
}

pub fn decode_head(buf: &[u8]) -> Option<PacketHead> {
    is_well_formed(buf)
        .then_some(PacketHead::decode(&buf[10..][..BE::read_u16(&buf[4..6]) as usize]).ok())
        .flatten()
}

pub fn is_well_formed(buf: &[u8]) -> bool {
    buf.len() >= OVERHEAD
        && &buf[0..2] == HEAD_MAGIC
        && OVERHEAD + BE::read_u16(&buf[4..6]) as usize + BE::read_u32(&buf[6..10]) as usize
            >= buf.len()
        && &buf[10 + BE::read_u16(&buf[4..6]) as usize + BE::read_u32(&buf[6..10]) as usize..][..2]
            == TAIL_MAGIC
}

include!("../gen/conversion.rs");
