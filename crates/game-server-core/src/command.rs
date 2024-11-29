use mavuika_message::output::ClientOutput;
use mavuika_persistence::player_information::PlayerInformation;
use mavuika_proto::PacketHead;

pub enum LogicCommand {
    CreateWorld {
        player_information: PlayerInformation,
        output: ClientOutput,
    },
    ClientInput {
        head: PacketHead,
        cmd_id: u16,
        data: Box<[u8]>,
        immediate_mode: bool,
    },
    WorldUpdate(u32),
}
