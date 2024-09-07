use crate::processing::{ParsedAccount, ParsedEvent, ParsedInstruction};
use crate::storage::{Account, StorageError, Transaction};
use scylla::{Session, SessionBuilder};

pub struct ScyllaStorage {
    session: Session,
}

impl ScyllaStorage {
    pub async fn new(nodes: &[String]) -> Result<Self, StorageError> {
        let session = SessionBuilder::new().known_nodes(nodes).build().await?;

        Ok(Self { session })
    }

    pub async fn store_account(
        &self,
        account: ParsedAccount,
        slot: u64,
    ) -> Result<(), StorageError> {
        let query =
            "INSERT INTO accounts (pubkey, owner, slot, account_type, data) VALUES (?, ?, ?, ?, ?)";
        match account {
            ParsedAccount::Token {
                pubkey,
                owner,
                data,
            } => {
                self.session
                    .query(
                        query,
                        (
                            pubkey.to_string(),
                            owner.to_string(),
                            slot,
                            "token",
                            serde_json::to_string(&data)?,
                        ),
                    )
                    .await?;
            }
            ParsedAccount::Unknown {
                pubkey,
                owner,
                data,
            } => {
                self.session
                    .query(
                        query,
                        (
                            pubkey.to_string(),
                            owner.to_string(),
                            slot,
                            "unknown",
                            serde_json::to_string(&data)?,
                        ),
                    )
                    .await?;
            } // Add other account types as needed
        }
        Ok(())
    }

    pub async fn store_instruction(
        &self,
        instruction: ParsedInstruction,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError> {
        let query = "INSERT INTO instructions (slot, tx_signature, program_id, instruction_type, data) VALUES (?, ?, ?, ?, ?)";
        match instruction {
            ParsedInstruction::Unknown { program_id, data } => {
                self.session
                    .query(
                        query,
                        (
                            slot,
                            tx_signature,
                            program_id.to_string(),
                            "unknown",
                            serde_json::to_string(&data)?,
                        ),
                    )
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn store_event(
        &self,
        event: ParsedEvent,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError> {
        let query = "INSERT INTO events (slot, tx_signature, event_type, data) VALUES (?, ?, ?, ?)";
        match event {
            ParsedEvent::Json(json) => {
                self.session
                    .query(
                        query,
                        (slot, tx_signature, "json", serde_json::to_string(&json)?),
                    )
                    .await?;
            }
            ParsedEvent::Plain(text) => {
                self.session
                    .query(query, (slot, tx_signature, "plain", text))
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn get_account(&self, pubkey: &str) -> Result<Option<Account>, StorageError> {
        let query = "SELECT data FROM accounts WHERE pubkey = ? ORDER BY slot DESC LIMIT 1";
        let result = self.session.query(query, (pubkey,)).await?;
        if let Some(row) = result.first() {
            let data: String = row.get_column("data")?;
            Ok(Some(serde_json::from_str(&data)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<Transaction>, StorageError> {
        let query = "SELECT data FROM transactions WHERE signature = ?";
        let result = self.session.query(query, (signature,)).await?;
        if let Some(row) = result.first() {
            let data: String = row.get_column("data")?;
            Ok(Some(serde_json::from_str(&data)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_transactions_by_account(
        &self,
        pubkey: &str,
        limit: i32,
    ) -> Result<Vec<Transaction>, StorageError> {
        let query = "SELECT data FROM transactions WHERE pubkey = ? ORDER BY slot DESC LIMIT ?";
        let result = self.session.query(query, (pubkey, limit)).await?;
        let transactions = result
            .rows
            .unwrap_or_default()
            .into_iter()
            .filter_map(|row| {
                let data: String = row.get_column("data").ok()?;
                serde_json::from_str(&data).ok()
            })
            .collect();
        Ok(transactions)
    }
}
