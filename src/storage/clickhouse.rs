use crate::processing::{ParsedAccount, ParsedEvent, ParsedInstruction};
use crate::storage::{Account, StorageError, Transaction};
use clickhouse::{Client, Row};

pub struct ClickhouseStorage {
    client: Client,
}

impl ClickhouseStorage {
    pub async fn new(url: &str) -> Result<Self, StorageError> {
        let client = Client::default()
            .with_url(url)
            .with_database("cypher_indexer")
            .await?;
        Ok(Self { client })
    }

    pub async fn store_account(
        &self,
        account: ParsedAccount,
        slot: u64,
    ) -> Result<(), StorageError> {
        let query =
            "INSERT INTO accounts (pubkey, owner, slot, account_type, data) VALUES (?, ?, ?, ?, ?)";
        let mut rows = Vec::new();
        match account {
            ParsedAccount::Token {
                pubkey,
                owner,
                data,
            } => {
                rows.push(Row::new(vec![
                    pubkey.to_string().into(),
                    owner.to_string().into(),
                    slot.into(),
                    "token".into(),
                    serde_json::to_string(&data)?.into(),
                ]));
            }
            ParsedAccount::Unknown {
                pubkey,
                owner,
                data,
            } => {
                rows.push(Row::new(vec![
                    pubkey.to_string().into(),
                    owner.to_string().into(),
                    slot.into(),
                    "unknown".into(),
                    serde_json::to_string(&data)?.into(),
                ]));
            }
        }
        self.client.insert(query, rows).await?;
        Ok(())
    }

    pub async fn store_instruction(
        &self,
        instruction: ParsedInstruction,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError> {
        let query = "INSERT INTO instructions (slot, tx_signature, program_id, instruction_type, data) VALUES (?, ?, ?, ?, ?)";
        let mut rows = Vec::new();
        match instruction {
            ParsedInstruction::Unknown { program_id, data } => {
                rows.push(Row::new(vec![
                    slot.into(),
                    tx_signature.into(),
                    program_id.to_string().into(),
                    "unknown".into(),
                    serde_json::to_string(&data)?.into(),
                ]));
            }
        }
        self.client.insert(query, rows).await?;
        Ok(())
    }

    pub async fn store_event(
        &self,
        event: ParsedEvent,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError> {
        let query = "INSERT INTO events (slot, tx_signature, event_type, data) VALUES (?, ?, ?, ?)";
        let mut rows = Vec::new();
        match event {
            ParsedEvent::Json(json) => {
                rows.push(Row::new(vec![
                    slot.into(),
                    tx_signature.into(),
                    "json".into(),
                    serde_json::to_string(&json)?.into(),
                ]));
            }
            ParsedEvent::Plain(text) => {
                rows.push(Row::new(vec![
                    slot.into(),
                    tx_signature.into(),
                    "plain".into(),
                    text.into(),
                ]));
            }
        }
        self.client.insert(query, rows).await?;
        Ok(())
    }

    pub async fn get_account(&self, pubkey: &str) -> Result<Option<Account>, StorageError> {
        let query = "SELECT * FROM accounts WHERE pubkey = ? ORDER BY slot DESC LIMIT 1";
        let mut cursor = self.client.query(query).bind(&(pubkey,)).execute().await?;
        if let Some(row) = cursor.next().await? {
            Ok(Some(serde_json::from_str(&row.get::<String, _>("data")?)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<Transaction>, StorageError> {
        let query = "SELECT * FROM transactions WHERE signature = ?";
        let mut cursor = self
            .client
            .query(query)
            .bind(&(signature,))
            .execute()
            .await?;
        if let Some(row) = cursor.next().await? {
            Ok(Some(serde_json::from_str(&row.get::<String, _>("data")?)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_transactions_by_account(
        &self,
        pubkey: &str,
        limit: i32,
    ) -> Result<Vec<Transaction>, StorageError> {
        let query = "SELECT * FROM transactions WHERE pubkey = ? ORDER BY slot DESC LIMIT ?";
        let mut cursor = self
            .client
            .query(query)
            .bind(&(pubkey, limit))
            .execute()
            .await?;
        let mut transactions = Vec::new();
        while let Some(row) = cursor.next().await? {
            transactions.push(serde_json::from_str(&row.get::<String, _>("data")?)?);
        }
        Ok(transactions)
    }
}
