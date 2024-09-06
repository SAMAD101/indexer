pub mod api;
pub mod config;
pub mod ingestion;
pub mod processing;
pub mod storage;
pub mod utils;
pub mod wasm;

use crate::api::ApiServer;
use crate::config::Config;
use crate::ingestion::{GeyserPlugin, RpcPoller, WebsocketListener};
use crate::processing::Processor;
use crate::storage::{ipfs::IpfsStorage, Storage};
use crate::wasm::runtime::WasmRuntime;

pub async fn run_indexer() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    let storage = Storage::new(&config).await?;
    let ipfs_storage = IpfsStorage::new(&config.ipfs_api_url);

    let mut wasm_runtime = WasmRuntime::new();

    let wasm_bytes = std::fs::read(&config.wasm_module_path)?;
    wasm_runtime.run_module(&wasm_bytes, "start", &[])?;

    let processor = Processor::new(storage.clone(), ipfs_storage);

    let geyser_plugin = GeyserPlugin::new(&config);
    let rpc_poller = RpcPoller::new(&config);
    let websocket_listener = WebsocketListener::new(&config);

    let api_server = ApiServer::new(storage.clone(), &config);

    tokio::try_join!(
        tokio::spawn(async move {
            if let Err(e) = geyser_plugin.start(processor.clone()).await {
                tracing::error!("Geyser plugin error: {:?}", e);
            }
        }),
        tokio::spawn(async move {
            if let Err(e) = rpc_poller.start(processor.clone()).await {
                tracing::error!("RPC poller error: {:?}", e);
            }
        }),
        tokio::spawn(async move {
            if let Err(e) = websocket_listener.start(processor).await {
                tracing::error!("WebSocket listener error: {:?}", e);
            }
        }),
        api_server.start(),
    )?;

    Ok(())
}
