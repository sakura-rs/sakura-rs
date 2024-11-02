use byteorder::{ByteOrder, BE};
use std::fmt;

pub struct RawPacket<'data>(&'data [u8]);

#[derive(thiserror::Error, Debug)]
#[error("input data is not a valid packet")]
pub struct PacketMalformedError;

impl<'data> RawPacket<'data> {
    pub fn new(data: &'data [u8]) -> Result<Self, PacketMalformedError> {
        crate::packet::is_well_formed(data)
            .then_some(Self(data))
            .ok_or(PacketMalformedError)
    }

    pub fn cmd_id(&self) -> u16 {
        crate::packet::read_cmd_id(self.0).unwrap()
    }

    pub fn head(&self) -> crate::PacketHead {
        use crate::Protobuf;

        let head_len = BE::read_u16(&self.0[4..6]) as usize;
        crate::PacketHead::decode(&self.0[10..][..head_len]).unwrap_or_default()
    }

    pub fn decode_as<T: crate::YSMessage + Default>(
        self,
    ) -> Result<crate::packet::NetPacket<T>, crate::packet::DecodeError> {
        crate::packet::NetPacket::decode(self.0)
    }

    pub fn body(&self) -> &[u8] {
        let head_len = BE::read_u16(&self.0[4..6]) as usize;
        let body_len = BE::read_u32(&self.0[6..10]) as usize;

        &self.0[10 + head_len..][..body_len]
    }
}

pub fn make_raw_packet(cmd_id: u16, head: crate::PacketHead, body: &[u8]) -> Box<[u8]> {
    use crate::Protobuf;

    let mut buf = vec![0u8; 12 + head.encoded_len() + body.len()].into_boxed_slice();
    let head_len = head.encoded_len();
    let body_len = body.len();

    buf[0..2].copy_from_slice(&0x4567_u16.to_be_bytes());
    BE::write_u16(&mut buf[2..4], cmd_id);
    BE::write_u16(&mut buf[4..6], head_len as u16);
    BE::write_u32(&mut buf[6..10], body_len as u32);

    head.encode(&mut buf[10..10 + head_len].as_mut()).unwrap();

    buf[10 + head_len..10 + head_len + body_len].copy_from_slice(body);

    buf[10 + head_len + body_len..12 + head_len + body_len]
        .copy_from_slice(&0x89AB_u16.to_be_bytes());

    buf
}

impl fmt::Display for RawPacket<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd_id = self.cmd_id();
        let head = self.head();
        let body = self.body();

        write!(
            f,
            "[cmd:{}|user:{}|session:{}|data:{}]",
            cmd_id,
            head.user_id,
            head.user_session_id,
            hex::encode(body)
        )
    }
}
