use common::TomlConfig;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DispatchConfig {
    pub http_addr: String,
    pub forbid_first_dispatch: bool,
    pub region: RegionConfig,
}

#[derive(Deserialize)]
pub struct RegionConfig {
    pub enable_login_pc: bool,
    pub region_list_file: String,
    pub client_custom_config_path: String,
    pub global_client_secret_key_path: String,
    pub encryption_config_path: String,
    pub cur_region_name: Option<String>,
}

impl TomlConfig for DispatchConfig {
    const DEFAULT_TOML: &str = include_str!("../dispatch-server.default.toml");
}
