pub const CONTROL_PACKET_SIZE: usize = 20;
pub struct ControlPacket([u8; CONTROL_PACKET_SIZE]);

impl ControlPacket {
    pub fn build(ty: ControlPacketType, conv: u32, token: u32, data: u32) -> Self {
        use byteorder::{ByteOrder, BE};

        let (head, tail) = ty.to_magic();
        let mut buf = [0u8; CONTROL_PACKET_SIZE];

        BE::write_u32(&mut buf[0..4], head);
        BE::write_u32(&mut buf[4..8], conv);
        BE::write_u32(&mut buf[8..12], token);
        BE::write_u32(&mut buf[12..16], data);
        BE::write_u32(&mut buf[16..20], tail);

        Self(buf)
    }

    pub fn get_type(&self) -> ControlPacketType {
        // never panics because there's no way to have invalid type at this point
        ControlPacketType::from_magic((
            u32::from_be_bytes(self.0[0..4].try_into().unwrap()),
            u32::from_be_bytes(self.0[16..20].try_into().unwrap()),
        ))
        .unwrap()
    }

    pub fn get_conv(&self) -> u32 {
        u32::from_be_bytes(self.0[4..8].try_into().unwrap())
    }

    pub fn get_token(&self) -> u32 {
        u32::from_be_bytes(self.0[8..12].try_into().unwrap())
    }

    pub fn get_data(&self) -> u32 {
        u32::from_be_bytes(self.0[12..16].try_into().unwrap())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlPacketType {
    Connect,
    Establish,
    Disconnect,
}

impl ControlPacketType {
    const CONNECT_MAGIC: (u32, u32) = (0xFF, 0xFFFFFFFF);
    const ESTABLISH_MAGIC: (u32, u32) = (0x145, 0x14514545);
    const DISCONNECT_MAGIC: (u32, u32) = (0x194, 0x19419494);

    pub fn to_magic(self) -> (u32, u32) {
        match self {
            Self::Connect => Self::CONNECT_MAGIC,
            Self::Establish => Self::ESTABLISH_MAGIC,
            Self::Disconnect => Self::DISCONNECT_MAGIC,
        }
    }

    pub fn from_magic(magic: (u32, u32)) -> Option<ControlPacketType> {
        Some(match magic {
            Self::CONNECT_MAGIC => Self::Connect,
            Self::ESTABLISH_MAGIC => Self::Establish,
            Self::DISCONNECT_MAGIC => Self::Disconnect,
            _ => return None,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TryFromError {
    #[error("unknown magic received: (0x{0:X}, 0x{1:X})")]
    UnknownMagic(u32, u32),
}

impl TryFrom<[u8; CONTROL_PACKET_SIZE]> for ControlPacket {
    type Error = TryFromError;

    fn try_from(value: [u8; CONTROL_PACKET_SIZE]) -> Result<Self, Self::Error> {
        let magic = (
            u32::from_be_bytes(value[0..4].try_into().unwrap()),
            u32::from_be_bytes(value[16..20].try_into().unwrap()),
        );

        ControlPacketType::from_magic(magic)
            .map(|_| Self(value))
            .ok_or(TryFromError::UnknownMagic(magic.0, magic.1))
    }
}
