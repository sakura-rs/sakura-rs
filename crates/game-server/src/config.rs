use common::TomlConfig;
use sakura_database::DatabaseSettings;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameServerConfig {
    pub database: DatabaseSettings,
    pub service_listen_addr: String,
    pub gate_server_addr: String,
}

impl TomlConfig for GameServerConfig {
    const DEFAULT_TOML: &str = include_str!("../game-server.default.toml");
}
