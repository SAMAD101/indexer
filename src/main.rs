mod config;
mod ingestion;
mod processing;
mod storage;
mod api;
mod utils;

use tokio;
use tracing_subscriber::{EnvFilter, fmt};
use crate::config::Config;
use crate::storage::Storage;
use crate::processing::Processor;
use crate::ingestion::{GeyserPlugin, RpcPoller, WebsocketListener};
use crate::api::ApiServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::load()?;

    let storage = Storage::new(&config).await?;

    let processor = Processor::new(storage.clone());

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