use common::TomlConfig;
use sakura_database::DatabaseSettings;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GateServerConfig {
    pub network: NetworkSettings,
    pub database: DatabaseSettings,
    pub cur_region_name: String,
    pub region_list_path: String,
    pub service_listen_addr: String,
    pub game_server_addr: String,
    pub encryption_config_path: String,
}

#[derive(Deserialize)]
pub struct NetworkSettings {
    pub udp_host: String,
}

impl TomlConfig for GateServerConfig {
    const DEFAULT_TOML: &str = include_str!("../gate-server.default.toml");
}
