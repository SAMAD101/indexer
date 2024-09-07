mod bigtable;
mod clickhouse;
pub mod ipfs;
mod redis;
mod scylla;

use crate::config::Config;
use crate::processing::{ParsedAccount, ParsedEvent, ParsedInstruction};
use async_trait::async_trait;

pub use self::bigtable::BigtableStorage;
pub use self::ipfs::IpfsStorage;

#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn store_account(&self, account: ParsedAccount, slot: u64) -> Result<(), StorageError>;
    async fn store_instruction(
        &self,
        instruction: ParsedInstruction,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError>;
    async fn store_event(
        &self,
        event: ParsedEvent,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError>;
    async fn get_account(&self, pubkey: &str) -> Result<Option<Account>, StorageError>;
    async fn get_transaction(&self, signature: &str) -> Result<Option<Transaction>, StorageError>;
    async fn get_transactions_by_account(
        &self,
        pubkey: &str,
        limit: i32,
    ) -> Result<Vec<Transaction>, StorageError>;
}

pub type Account = serde_json::Value;
pub type Transaction = serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("ClickHouse error: {0}")]
    Clickhouse(#[from] clickhouse::error::Error),
    #[error("ScyllaDB error: {0}")]
    Scylla(#[from] scylla::transport::errors::QueryError),
    #[error("IPFS error: {0}")]
    Ipfs(#[from] ipfs_api_backend_hyper::Error),
    #[error("BigTable error: {0}")]
    BigTable(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Clone)]
pub struct Storage {
    clickhouse: clickhouse::ClickhouseStorage,
    scylla: scylla::ScyllaStorage,
    redis: redis::RedisStorage,
    bigtable: BigtableStorage,
}

impl Storage {
    pub async fn new(config: &Config) -> Result<Self, StorageError> {
        Ok(Self {
            clickhouse: clickhouse::ClickhouseStorage::new(&config.clickhouse_url).await?,
            scylla: scylla::ScyllaStorage::new(&config.scylla_nodes).await?,
            redis: redis::RedisStorage::new(&config.redis_url).await?,
            bigtable: BigtableStorage::new(
                &config.bigtable_instance_name,
                &config.bigtable_app_profile_id,
            )
            .await?,
        })
    }

    pub async fn store_account(
        &self,
        account: ParsedAccount,
        slot: u64,
    ) -> Result<(), StorageError> {
        self.clickhouse.store_account(account.clone(), slot).await?;
        self.scylla.store_account(account.clone(), slot).await?;
        self.bigtable.store_account(account, slot).await?;
        Ok(())
    }

    pub async fn store_instruction(
        &self,
        instruction: ParsedInstruction,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError> {
        self.clickhouse
            .store_instruction(instruction.clone(), slot, tx_signature)
            .await?;
        self.scylla
            .store_instruction(instruction.clone(), slot, tx_signature)
            .await?;
        self.bigtable
            .store_instruction(instruction, slot, tx_signature)
            .await?;
        Ok(())
    }

    pub async fn store_event(
        &self,
        event: ParsedEvent,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError> {
        self.clickhouse
            .store_event(event.clone(), slot, tx_signature)
            .await?;
        self.scylla
            .store_event(event.clone(), slot, tx_signature)
            .await?;
        self.bigtable.store_event(event, slot, tx_signature).await?;
        Ok(())
    }

    pub async fn get_account(&self, pubkey: &str) -> Result<Option<Account>, StorageError> {
        if let Some(account) = self.redis.get_account(pubkey).await? {
            return Ok(Some(account));
        }
        let account = self.bigtable.get_account(pubkey).await?;
        if let Some(ref account) = account {
            self.redis.set_account(pubkey, account).await?;
        }
        Ok(account)
    }

    pub async fn get_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<Transaction>, StorageError> {
        if let Some(transaction) = self.redis.get_transaction(signature).await? {
            return Ok(Some(transaction));
        }
        let transaction = self.bigtable.get_transaction(signature).await?;
        if let Some(ref transaction) = transaction {
            self.redis.set_transaction(signature, transaction).await?;
        }
        Ok(transaction)
    }

    pub async fn get_transactions_by_account(
        &self,
        pubkey: &str,
        limit: i32,
    ) -> Result<Vec<Transaction>, StorageError> {
        self.bigtable
            .get_transactions_by_account(pubkey, limit)
            .await
    }
}
