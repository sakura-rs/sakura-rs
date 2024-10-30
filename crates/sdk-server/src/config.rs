use common::TomlConfig;
use sakura_database::DatabaseSettings;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SdkServerConfig {
    pub http_addr: String,
    pub database: DatabaseSettings,
}

impl TomlConfig for SdkServerConfig {
    const DEFAULT_TOML: &str = include_str!("../sdk-server.default.toml");
}
