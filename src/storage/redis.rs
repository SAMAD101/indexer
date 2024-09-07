use crate::storage::{Account, StorageError, Transaction};
use redis::{AsyncCommands, Client};

pub struct RedisStorage {
    client: Client,
}

impl RedisStorage {
    pub async fn new(url: &str) -> Result<Self, StorageError> {
        let client = Client::open(url)?;
        Ok(Self { client })
    }

    pub async fn get_account(&self, pubkey: &str) -> Result<Option<Account>, StorageError> {
        let mut conn = self.client.get_async_connection().await?;
        let data: Option<String> = conn.get(format!("account:{}", pubkey)).await?;
        Ok(data.map(|d| serde_json::from_str(&d).unwrap()))
    }

    pub async fn set_account(&self, pubkey: &str, account: &Account) -> Result<(), StorageError> {
        let mut conn = self.client.get_async_connection().await?;
        conn.set_ex(
            format!("account:{}", pubkey),
            serde_json::to_string(account)?,
            3600,
        )
        .await?;
        Ok(())
    }

    pub async fn get_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<Transaction>, StorageError> {
        let mut conn = self.client.get_async_connection().await?;
        let data: Option<String> = conn.get(format!("tx:{}", signature)).await?;
        Ok(data.map(|d| serde_json::from_str(&d).unwrap()))
    }

    pub async fn set_transaction(
        &self,
        signature: &str,
        transaction: &Transaction,
    ) -> Result<(), StorageError> {
        let mut conn = self.client.get_async_connection().await?;
        conn.set_ex(
            format!("tx:{}", signature),
            serde_json::to_string(transaction)?,
            3600,
        )
        .await?;
        Ok(())
    }
}
