use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub solana_rpc_url: String,
    pub clickhouse_url: String,
    pub scylla_nodes: Vec<String>,
    pub redis_url: String,
    pub ipfs_api_url: String,
    pub wasm_module_path: String,
    pub geyser_plugin_config: GeyserPluginConfig,
    pub rpc_poll_interval: u64,
    pub websocket_url: String,
    pub(crate) wasm_modules: Option<_>,
    pub(crate) wasm_memory_limit: Option<i32>,
    pub(crate) wasm_execution_timeout: Option<i32>,
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

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}
