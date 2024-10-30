mod config;
mod handlers;
mod util;

use std::sync::OnceLock;

use anyhow::Result;
use axum::Router;
use common::{logging, TomlConfig};
use config::SdkServerConfig;
use handlers::{combo_granter, mdk_shield_api, register, risky_api};
use sakura_database::DbConnection;
use tracing::Level;

#[derive(Clone)]
struct AppState {
    db: &'static DbConnection,
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: OnceLock<SdkServerConfig> = OnceLock::new();
    static DATABASE: OnceLock<DbConnection> = OnceLock::new();

    logging::init(Level::DEBUG);
    let config = CONFIG.get_or_init(|| SdkServerConfig::load_or_create("sdk-server.toml"));

    let database = sakura_database::connect_to(&config.database).await?;
    let database = DATABASE.get_or_init(move || database);

    sakura_database::run_migrations(database).await?;

    let app = Router::new()
        .merge(risky_api::routes())
        .merge(register::routes())
        .merge(mdk_shield_api::routes())
        .merge(combo_granter::routes())
        .with_state(AppState { db: database });

    axum_server::bind(config.http_addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
