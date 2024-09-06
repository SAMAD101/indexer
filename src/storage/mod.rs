mod clickhouse;
pub mod ipfs;
mod redis;
mod scylla;
use crate::processing::{ParsedAccount, ParsedEvent, ParsedInstruction};
use async_trait::async_trait;

use crate::config::Config;

#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn store_account(
        &self,
        account: ParsedAccount,
        slot: u64,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn store_instruction(
        &self,
        instruction: ParsedInstruction,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn store_event(
        &self,
        event: ParsedEvent,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_account(
        &self,
        pubkey: &str,
    ) -> Result<Option<Account>, Box<dyn std::error::Error>>;
    async fn get_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<Transaction>, Box<dyn std::error::Error>>;
    async fn get_transactions_by_account(
        &self,
        pubkey: &str,
        limit: i32,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>>;
}

pub type Account = serde_json::Value;
pub type Transaction = serde_json::Value;

#[derive(Clone, Debug)]
pub struct StorageError {
    message: String,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StorageError {}

impl From<Box<dyn std::error::Error>> for StorageError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        StorageError {
            message: error.to_string(),
        }
    }
}

impl From<redis::RedisError> for StorageError {
    fn from(error: redis::RedisError) -> Self {
        StorageError {
            message: error.to_string(),
        }
    }
}

impl From<clickhouse::ClickhouseError> for StorageError {
    fn from(error: clickhouse::ClickhouseError) -> Self {
        StorageError {
            message: error.to_string(),
        }
    }
}

impl From<scylla::ScyllaError> for StorageError {
    fn from(error: scylla::ScyllaError) -> Self {
        StorageError {
            message: error.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Storage {
    clickhouse: clickhouse::ClickhouseStorage,
    scylla: scylla::ScyllaStorage,
    redis: redis::RedisStorage,
}

impl Storage {
    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            clickhouse: clickhouse::ClickhouseStorage::new(&config.clickhouse_url).await?,
            scylla: scylla::ScyllaStorage::new(&config.scylla_nodes).await?,
            redis: redis::RedisStorage::new(&config.redis_url).await?,
        })
    }

    pub async fn store_account(
        &self,
        account: ParsedAccount,
        slot: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clickhouse.store_account(account.clone(), slot).await?;
        self.scylla.store_account(account, slot).await?;
        Ok(())
    }

    pub async fn store_instruction(
        &self,
        instruction: ParsedInstruction,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clickhouse
            .store_instruction(instruction.clone(), slot, tx_signature)
            .await?;
        self.scylla
            .store_instruction(instruction, slot, tx_signature)
            .await?;
        Ok(())
    }

    pub async fn store_event(
        &self,
        event: ParsedEvent,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clickhouse
            .store_event(event.clone(), slot, tx_signature)
            .await?;
        self.scylla.store_event(event, slot, tx_signature).await?;
        Ok(())
    }

    pub async fn get_account(
        &self,
        pubkey: &str,
    ) -> Result<Option<Account>, Box<dyn std::error::Error>> {
        if let Some(account) = self.redis.get_account(pubkey).await? {
            return Ok(Some(account));
        }
        let account = self.clickhouse.get_account(pubkey).await?;
        if let Some(ref account) = account {
            self.redis.set_account(pubkey, account).await?;
        }
        Ok(account)
    }

    pub async fn get_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<Transaction>, Box<dyn std::error::Error>> {
        if let Some(transaction) = self.redis.get_transaction(signature).await? {
            return Ok(Some(transaction));
        }
        let transaction = self.clickhouse.get_transaction(signature).await?;
        if let Some(ref transaction) = transaction {
            self.redis.set_transaction(signature, transaction).await?;
        }
        Ok(transaction)
    }

    pub async fn get_transactions_by_account(
        &self,
        pubkey: &str,
        limit: i32,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        self.clickhouse
            .get_transactions_by_account(pubkey, limit)
            .await
    }
}
