use std::sync::{LazyLock, OnceLock};

use anyhow::Result;
use common::{logging, TomlConfig};
use config::GameServerConfig;
use db_worker::DbWorkerHandle;
use game_server_core::LogicSimulator;
use mavuika_data::{config::load_configs_from_binary, excel};
use mavuika_network::{listener, ServerSocket};
use tracing::Level;

mod config;
mod db_worker;
mod message_handler;
mod player_info_util;

struct AppState {
    pub db_handle: DbWorkerHandle,
    pub logic_simulator: LogicSimulator,
    pub gate_server_socket: ServerSocket,
}

#[tokio::main]
async fn main() -> Result<()> {
    static STATE: OnceLock<AppState> = OnceLock::new();
    static CONFIG: LazyLock<GameServerConfig> =
        LazyLock::new(|| GameServerConfig::load_or_create("game-server.toml"));
    logging::init(Level::DEBUG);

    excel::load_all("assets/ExcelBinOutput")?;
    load_configs_from_binary("assets/BinOutput")?;

    let db_connection = mavuika_database::connect_to(&CONFIG.database).await?;
    mavuika_database::run_migrations(&db_connection).await?;
    let (db_handle, save_data_tx) = db_worker::start(db_connection);

    let gate_server_socket = ServerSocket::new(&CONFIG.gate_server_addr);

    let state = STATE.get_or_init(move || AppState {
        db_handle,
        logic_simulator: LogicSimulator::spawn(save_data_tx),
        gate_server_socket,
    });

    let _ = listener::listen(
        &CONFIG.service_listen_addr,
        state,
        message_handler::on_message,
    )
    .await?
    .await;

    Ok(())
}
