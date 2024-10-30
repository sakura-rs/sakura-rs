use sakura_message::output::ClientOutput;
use sakura_persistence::player_information::PlayerInformation;

pub enum LogicCommand {
    CreateWorld {
        player_information: PlayerInformation,
        output: ClientOutput,
    },
    ClientInput {
        uid: u32,
        cmd_id: u16,
        data: Box<[u8]>,
        immediate_mode: bool,
    },
    WorldUpdate(u32),
}
