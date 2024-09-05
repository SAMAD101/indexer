use solana_client::rpc_client::RpcClient;
use solana_transaction_status::UiConfirmedBlock;
use crate::config::Config;
use crate::processing::Processor;
use tokio::time::{interval, Duration};
use async_trait::async_trait;

pub struct RpcPoller {
    rpc_client: RpcClient,
    poll_interval: Duration,
}

impl RpcPoller {
    pub fn new(config: &Config) -> Self {
        Self {
            rpc_client: RpcClient::new(config.solana_rpc_url.clone()),
            poll_interval: Duration::from_secs(config.rpc_poll_interval),
        }
    }
}

#[async_trait]
impl super::IngestionSource for RpcPoller {
    async fn start(&self, processor: Processor) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = interval(self.poll_interval);
        let mut last_slot = self.rpc_client.get_slot()?;

        loop {
            interval.tick().await;
            let current_slot = self.rpc_client.get_slot()?;

            for slot in last_slot + 1..=current_slot {
                let block: UiConfirmedBlock = self.rpc_client.get_block_with_encoding(
                    slot,
                    solana_transaction_status::UiTransactionEncoding::Json
                )?;
                
                for transaction in block.transactions {
                    if let Some(transaction) = transaction {
                        processor.process_transaction(transaction, slot).await?;
                    }
                }
            }

            last_slot = current_slot;
        }
    }
}