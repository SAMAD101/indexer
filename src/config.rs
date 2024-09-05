use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub solana_rpc_url: String,
    pub clickhouse_url: String,
    pub scylla_nodes: Vec<String>,
    pub redis_url: String,
    pub geyser_plugin_config: GeyserPluginConfig,
    pub rpc_poll_interval: u64,
    pub websocket_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GeyserPluginConfig {
    pub libpath: String,
    pub accounts_selector: AccountsSelector,
}

#[derive(Debug, Deserialize)]
pub struct AccountsSelector {
    pub owners: Vec<String>,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}