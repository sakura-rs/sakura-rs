use std::{
    collections::HashMap,
    fs,
    sync::{Arc, LazyLock, OnceLock},
};

use anyhow::Result;
use common::{
    data::{EncryptionConfig, RegionConfig},
    logging, TomlConfig,
};
use config::GateServerConfig;
use dashmap::DashMap;
use sakura_database::DbConnection;
use sakura_encryption::{rsa::RsaKeyPair, xor::MhyXorpad};
use sakura_network::{listener, ServerSocket};
use handler::server_message_handler;
use net::UdpServer;
use tracing::Level;

mod config;
mod handler;
mod net;
mod util;

struct AppState {
    region_config: RegionConfig,
    db_connection: DbConnection,
    sessions: DashMap<u32, Arc<handler::Session>>,
    key_pair_map: HashMap<u32, RsaKeyPair>,
    initial_xorpad: Option<MhyXorpad>,
    game_server_socket: ServerSocket,
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<GateServerConfig> =
        LazyLock::new(|| GateServerConfig::load_or_create("gate-server.toml"));
    static STATE: OnceLock<AppState> = OnceLock::new();

    logging::init(Level::DEBUG);

    let region_list: Vec<RegionConfig> =
        serde_json::from_str(&fs::read_to_string(&CONFIG.region_list_path)?)?;
    let key_pair_map = serde_json::from_str::<HashMap<u32, EncryptionConfig>>(
        &fs::read_to_string(&CONFIG.encryption_config_path)?,
    )?
    .into_iter()
    .map(|(id, conf)| (id, RsaKeyPair::from_encryption_config(&conf)))
    .collect();

    let cur_region = region_list
        .into_iter()
        .find(|r| &r.name == &CONFIG.cur_region_name)
        .expect("cur_region not found in region list");

    let initial_xorpad = if let Some(secret_key_path) = cur_region.secret_key_path.as_ref() {
        Some(MhyXorpad::from_ec2b(&fs::read(secret_key_path)?)?)
    } else {
        None
    };

    let db_connection = sakura_database::connect_to(&CONFIG.database).await?;
    sakura_database::run_migrations(&db_connection).await?;

    let game_server_socket = ServerSocket::new(&CONFIG.game_server_addr);

    let state = STATE.get_or_init(move || AppState {
        region_config: cur_region,
        db_connection,
        sessions: DashMap::new(),
        initial_xorpad,
        key_pair_map,
        game_server_socket,
    });

    listener::listen(
        &CONFIG.service_listen_addr,
        state,
        server_message_handler::on_message,
    )
    .await?;

    let udp_server = UdpServer::bind(CONFIG.network.udp_host.parse()?, state).await?;
    udp_server.serve().await;

    Ok(())
}
