mod config;
mod handlers;

use std::{
    collections::HashMap,
    fs,
    sync::{LazyLock, OnceLock},
};

use anyhow::{anyhow, Result};
use axum::Router;
use common::{
    data::{EncryptionConfig, RegionConfig},
    logging, TomlConfig,
};
use config::DispatchConfig;
use mavuika_encryption::{rsa::RsaKeyPair, xor::MhyXorpad};
use tracing::Level;

struct AppState {
    pub config: &'static DispatchConfig,
    pub region_list: Vec<RegionConfig>,
    pub key_pair_map: HashMap<u32, RsaKeyPair>,
    pub global_secret_key_ec2b: Box<[u8]>,
    pub client_custom_config_encrypted: Box<[u8]>,
    pub cur_region_secret_key_ec2b: Option<Box<[u8]>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<DispatchConfig> =
        LazyLock::new(|| DispatchConfig::load_or_create("dispatch-server.toml"));
    static STATE: OnceLock<AppState> = OnceLock::new();

    logging::init(Level::DEBUG);

    let global_ec2b = fs::read(&CONFIG.region.global_client_secret_key_path)
        .map_err(|err| {
            anyhow!(
                "failed to read global secret key from {}, error: {err}",
                &CONFIG.region.global_client_secret_key_path
            )
        })?
        .into_boxed_slice();

    let global_xorpad = MhyXorpad::from_ec2b(&global_ec2b).map_err(|err| {
        anyhow!(
            "failed to parse ec2b from {}, error: {err}",
            &CONFIG.region.global_client_secret_key_path
        )
    })?;

    let client_custom_config_encrypted = fs::read(&CONFIG.region.client_custom_config_path)
        .map(|mut c| {
            global_xorpad.xor(&mut c);
            c.into_boxed_slice()
        })
        .map_err(|err| {
            anyhow!(
                "failed to read client_custom_config from {}, error: {err}",
                &CONFIG.region.client_custom_config_path
            )
        })?;

    let region_list: Vec<RegionConfig> =
        serde_json::from_str(&fs::read_to_string(&CONFIG.region.region_list_file)?)?;

    let cur_region_secret_key_ec2b = if let Some(cur_region_name) = &CONFIG.region.cur_region_name {
        let region = region_list
            .iter()
            .find(|r| &r.name == cur_region_name)
            .ok_or_else(|| {
                anyhow!("can't find find cur_region_name {cur_region_name} in region_list")
            })?;

        match region.secret_key_path.as_ref().map(|path| {
            fs::read(path).map_err(|err| {
                anyhow!("failed to read region-local secret key from {path}, error: {err}")
            })
        }) {
            Some(Ok(data)) => Some(data.into_boxed_slice()),
            Some(Err(err)) => return Err(err),
            None => None,
        }
    } else {
        None
    };

    let key_pair_map = serde_json::from_str::<HashMap<u32, EncryptionConfig>>(
        &fs::read_to_string(&CONFIG.region.encryption_config_path)?,
    )?
    .into_iter()
    .map(|(id, conf)| (id, RsaKeyPair::from_encryption_config(&conf)))
    .collect();

    let state = STATE.get_or_init(move || AppState {
        config: &CONFIG,
        region_list,
        global_secret_key_ec2b: global_ec2b,
        client_custom_config_encrypted,
        cur_region_secret_key_ec2b,
        key_pair_map,
    });

    let app = Router::new().merge(handlers::routes()).with_state(state);

    axum_server::bind(CONFIG.http_addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
