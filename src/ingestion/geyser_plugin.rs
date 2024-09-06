use crate::config::Config;
use crate::processing::Processor;
use async_trait::async_trait;
use parking_lot::RwLock;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
    ReplicaTransactionInfoVersions, SlotStatus,
};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Error)]
pub enum GeyserPluginError {
    #[error("Processing error: {0}")]
    ProcessingError(String),
    #[error("Channel send error: {0}")]
    ChannelSendError(#[from] tokio::sync::mpsc::error::SendError<GeyserEvent>),
}

pub struct GeyserPlugin {
    config: Config,
    processor: Arc<RwLock<Option<Processor>>>,
    event_sender: mpsc::Sender<GeyserEvent>,
}

#[derive(Debug)]
enum GeyserEvent {
    AccountUpdate(Box<ReplicaAccountInfoVersions<'static>>, u64, bool),
    TransactionNotify(Box<ReplicaTransactionInfoVersions<'static>>, u64),
    BlockMetadata(Box<ReplicaBlockInfoVersions<'static>>),
    SlotStatusChange(u64, Option<u64>, SlotStatus),
}

impl GeyserPlugin {
    pub fn new(config: &Config) -> Self {
        let (event_sender, mut event_receiver) = mpsc::channel(1000); // Adjust buffer size as needed
        let processor = Arc::new(RwLock::new(None));
        let processor_clone = Arc::clone(&processor);

        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                if let Some(processor) = processor_clone.read().as_ref() {
                    match event {
                        GeyserEvent::AccountUpdate(account, slot, is_startup) => {
                            let _ = processor
                                .process_account_update(account, slot, is_startup)
                                .await;
                        }
                        GeyserEvent::TransactionNotify(transaction, slot) => {
                            let _ = processor.process_transaction(transaction, slot).await;
                        }
                        GeyserEvent::BlockMetadata(block_info) => {
                            let _ = processor.process_block_metadata(block_info).await;
                        }
                        GeyserEvent::SlotStatusChange(slot, parent, status) => {
                            let _ = processor.process_slot_status(slot, parent, status).await;
                        }
                    }
                }
            }
        });

        Self {
            config: config.clone(),
            processor,
            event_sender,
        }
    }

    fn send_event(&self, event: GeyserEvent) -> Result<(), GeyserPluginError> {
        self.event_sender.try_send(event).map_err(Into::into)
    }
}

#[async_trait]
impl super::IngestionSource for GeyserPlugin {
    async fn start(&self, processor: Processor) -> Result<(), Box<dyn std::error::Error>> {
        *self.processor.write() = Some(processor);
        Ok(())
    }
}

impl GeyserPlugin for GeyserPlugin {
    fn name(&self) -> &'static str {
        "CypherIndexerGeyserPlugin"
    }

    fn on_load(
        &mut self,
        _config_file: &str,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        // Initialize plugin
        Ok(())
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let account = Box::new(unsafe { std::mem::transmute(account) });
        self.send_event(GeyserEvent::AccountUpdate(account, slot, is_startup))
            .map_err(|e| {
                solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPluginError::Custom(
                    Box::new(e),
                )
            })
    }

    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let transaction = Box::new(unsafe { std::mem::transmute(transaction) });
        self.send_event(GeyserEvent::TransactionNotify(transaction, slot))
            .map_err(|e| {
                solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPluginError::Custom(
                    Box::new(e),
                )
            })
    }

    fn notify_block_metadata(
        &self,
        blockinfo: ReplicaBlockInfoVersions,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let blockinfo = Box::new(unsafe { std::mem::transmute(blockinfo) });
        self.send_event(GeyserEvent::BlockMetadata(blockinfo))
            .map_err(|e| {
                solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPluginError::Custom(
                    Box::new(e),
                )
            })
    }

    fn update_slot_status(
        &self,
        slot: u64,
        parent: Option<u64>,
        status: SlotStatus,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        self.send_event(GeyserEvent::SlotStatusChange(slot, parent, status))
            .map_err(|e| {
                solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPluginError::Custom(
                    Box::new(e),
                )
            })
    }

    fn on_unload(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_geyser_plugin_creation() {
        let config = Config::default();
        let plugin = GeyserPlugin::new(&config);
        assert_eq!(plugin.name(), "CypherIndexerGeyserPlugin");
    }
}
