pub use prost::DecodeError as ProtobufDecodeError;
pub use prost::Message as Protobuf;

pub trait CmdID {
    const CMD_ID: u16;

    fn get_cmd_id(&self) -> u16 {
        Self::CMD_ID
    }
}

pub trait YSMessage: Protobuf + CmdID {}
impl<T: Protobuf + CmdID> YSMessage for T {}

mod client;
mod normal;
pub mod packet;
pub mod raw_packet;
mod retcode;

pub use normal::*;
pub use retcode::Retcode;
