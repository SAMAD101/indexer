use crate::processing::{ParsedAccount, ParsedEvent, ParsedInstruction};
use crate::storage::{Account, StorageError, Transaction};
use serde::{Deserialize, Serialize};
use solana_bigtable_connection::{
    bigtable::{BigTableConnection, RowData, RowKey},
    CredentialType,
};

const ACCOUNT_TABLE: &str = "accounts";
const INSTRUCTION_TABLE: &str = "instructions";
const EVENT_TABLE: &str = "events";
const TRANSACTION_TABLE: &str = "transactions";

pub struct BigtableStorage {
    connection: BigTableConnection,
}

impl BigtableStorage {
    pub async fn new(instance_name: &str, app_profile_id: &str) -> Result<Self, StorageError> {
        let connection = BigTableConnection::new(
            instance_name,
            app_profile_id,
            false,
            None,
            CredentialType::Filepath(None),
        )
        .await
        .map_err(|e| StorageError::Other(e.to_string()))?;

        Ok(Self { connection })
    }

    pub async fn store_account(
        &self,
        account: ParsedAccount,
        slot: u64,
    ) -> Result<(), StorageError> {
        let row_key = format!("{}-{}", account.pubkey(), slot);
        let serialized =
            bincode::serialize(&account).map_err(|e| StorageError::Serialization(e.into()))?;

        self.connection
            .put_bincode_cells_with_retry(ACCOUNT_TABLE, &[(row_key.into(), serialized)], true)
            .await
            .map_err(|e| StorageError::Other(e.to_string()))?;

        Ok(())
    }

    pub async fn store_instruction(
        &self,
        instruction: ParsedInstruction,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError> {
        let row_key = format!("{}-{}-{}", tx_signature, slot, instruction.program_id());
        let serialized =
            bincode::serialize(&instruction).map_err(|e| StorageError::Serialization(e.into()))?;

        self.connection
            .put_bincode_cells_with_retry(INSTRUCTION_TABLE, &[(row_key.into(), serialized)], true)
            .await
            .map_err(|e| StorageError::Other(e.to_string()))?;

        Ok(())
    }

    pub async fn store_event(
        &self,
        event: ParsedEvent,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), StorageError> {
        let row_key = format!("{}-{}", tx_signature, slot);
        let serialized =
            bincode::serialize(&event).map_err(|e| StorageError::Serialization(e.into()))?;

        self.connection
            .put_bincode_cells_with_retry(EVENT_TABLE, &[(row_key.into(), serialized)], true)
            .await
            .map_err(|e| StorageError::Other(e.to_string()))?;

        Ok(())
    }

    pub async fn get_account(&self, pubkey: &str) -> Result<Option<Account>, StorageError> {
        let mut client = self.connection.client();
        let result = client
            .get_bincode_cell::<Account>(ACCOUNT_TABLE, pubkey.into())
            .await
            .map_err(|e| StorageError::Other(e.to_string()))?;

        Ok(Some(result))
    }

    pub async fn get_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<Transaction>, StorageError> {
        let mut client = self.connection.client();
        let result = client
            .get_bincode_cell::<Transaction>(TRANSACTION_TABLE, signature.into())
            .await
            .map_err(|e| StorageError::Other(e.to_string()))?;

        Ok(Some(result))
    }

    pub async fn get_transactions_by_account(
        &self,
        pubkey: &str,
        limit: i32,
    ) -> Result<Vec<Transaction>, StorageError> {
        let mut client = self.connection.client();
        let row_data = client
            .get_row_data(TRANSACTION_TABLE, Some(pubkey.into()), None, limit as i64)
            .await
            .map_err(|e| StorageError::Other(e.to_string()))?;

        let transactions: Vec<Transaction> = row_data
            .into_iter()
            .filter_map(|(_, data)| bincode::deserialize(&data[0].1).ok())
            .collect();

        Ok(transactions)
    }
}
