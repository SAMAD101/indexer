use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    rpc_response::RpcLogsResponse,
};
use crate::config::Config;
use crate::processing::Processor;
use async_trait::async_trait;
use futures::StreamExt;

pub struct WebsocketListener {
    rpc_client: RpcClient,
}

impl WebsocketListener {
    pub fn new(config: &Config) -> Self {
        Self {
            rpc_client: RpcClient::new(config.solana_rpc_url.clone()),
        }
    }
}

#[async_trait]
impl super::IngestionSource for WebsocketListener {
    async fn start(&self, processor: Processor) -> Result<(), Box<dyn std::error::Error>> {
        let logs_config = RpcTransactionLogsConfig {
            commitment: Some(solana_sdk::commitment_config::CommitmentConfig::confirmed()),
            kind: RpcTransactionLogsFilter::All,
        };

        let (mut logs_notifications, logs_unsubscribe) = self.rpc_client
            .on_logs_event(logs_config)
            .await?;

        while let Some(logs) = logs_notifications.next().await {
            match logs {
                RpcLogsResponse::Logs { signature, logs, .. } => {
                    let transaction = self.rpc_client.get_transaction(&signature, solana_transaction_status::UiTransactionEncoding::Json).await?;
                    processor.process_transaction(transaction, logs).await?;
                }
                _ => {}
            }
        }

        logs_unsubscribe().await?;
        Ok(())
    }
}