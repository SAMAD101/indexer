mod geyser_plugin;
mod rpc_poller;
mod websocket_listener;

pub use geyser_plugin::GeyserPlugin;
pub use rpc_poller::RpcPoller;
pub use websocket_listener::WebsocketListener;

use async_trait::async_trait;
use crate::processing::Processor;

#[async_trait]
pub trait IngestionSource: Send + Sync {
    async fn start(&self, processor: Processor) -> Result<(), Box<dyn std::error::Error>>;
}