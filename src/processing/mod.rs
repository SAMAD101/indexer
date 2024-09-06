use crate::storage::ipfs::IpfsStorage;
use crate::storage::Storage;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{EncodedConfirmedTransaction, UiConfirmedBlock};

pub mod account_parser;
pub mod event_parser;
pub mod instruction_parser;
pub mod state_manager;

use account_parser::{AccountParser, ParsedAccount};
use event_parser::{EventParser, ParsedEvent};
use instruction_parser::{InstructionParser, ParsedInstruction};
use state_manager::StateManager;

#[derive(Clone)]
pub struct Processor {
    storage: Storage,
    ipfs_storage: IpfsStorage,
    account_parser: AccountParser,
    instruction_parser: InstructionParser,
    event_parser: EventParser,
    state_manager: StateManager,
}

impl Processor {
    pub fn new(storage: Storage, ipfs_storage: IpfsStorage) -> Self {
        Self {
            storage,
            ipfs_storage,
            account_parser: AccountParser::new(),
            instruction_parser: InstructionParser::new(),
            event_parser: EventParser::new(),
            state_manager: StateManager::new(),
        }
    }

    pub async fn process_transaction(
        &self,
        transaction: EncodedConfirmedTransaction,
        slot: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let signature = transaction.transaction.signatures[0].to_string();

        for (index, instruction) in transaction
            .transaction
            .message
            .instructions
            .iter()
            .enumerate()
        {
            let program_id =
                transaction.transaction.message.account_keys[instruction.program_id_index as usize];
            let parsed_instruction = self
                .instruction_parser
                .parse_instruction(&program_id, instruction)?;
            self.storage
                .store_instruction(parsed_instruction, slot, &signature)
                .await?;
        }

        if let Some(log_messages) = transaction.meta.log_messages {
            let events = self.event_parser.parse_logs(&log_messages)?;
            for event in events {
                self.storage.store_event(event, slot, &signature).await?;
            }
        }

        if let Some(post_balances) = transaction.meta.post_balances {
            for (index, &lamports) in post_balances.iter().enumerate() {
                if let Some(account_keys) = &transaction.transaction.message.account_keys {
                    let pubkey = &account_keys[index];
                    if let Some(account_data) = &transaction.meta.post_token_balances {
                        let parsed_account = self.account_parser.parse_account(
                            pubkey,
                            account_data,
                            &Pubkey::default(),
                        )?;
                        self.state_manager
                            .update_account(*pubkey, parsed_account.clone());
                        self.storage.store_account(parsed_account, slot).await?;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn process_block(
        &self,
        block: UiConfirmedBlock,
        slot: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for transaction in block.transactions {
            if let Some(transaction) = transaction {
                self.process_transaction(transaction, slot).await?;
            }
        }
        Ok(())
    }

    pub async fn process_account_update(
        &self,
        pubkey: Pubkey,
        data: Vec<u8>,
        owner: Pubkey,
        slot: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let parsed_account = self.account_parser.parse_account(&pubkey, &data, &owner)?;
        self.state_manager
            .update_account(pubkey, parsed_account.clone());
        self.storage.store_account(parsed_account, slot).await?;
        Ok(())
    }

    pub async fn process_instruction(
        &self,
        instruction: ParsedInstruction,
        slot: u64,
        tx_signature: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match instruction {
            ParsedInstruction::CypherInitialize { params } => {
                self.storage
                    .store_instruction(instruction.clone(), slot, tx_signature)
                    .await?;
                self.state_manager.update_account(
                    params.mint_authority,
                    ParsedAccount::CypherMint {
                        pubkey: params.mint_authority,
                        data: CypherMintData {
                            supply: 0,
                            decimals: params.decimals,
                            mint_authority: params.mint_authority,
                            freeze_authority: params.freeze_authority,
                        },
                    },
                );
            }
            ParsedInstruction::CypherTransfer { params } => {
                self.storage
                    .store_instruction(instruction.clone(), slot, tx_signature)
                    .await?;
                let mint_account = self
                    .state_manager
                    .get_account(&params.mint)
                    .ok_or(Box::new(ProcessorError::InvalidAccount))?;
                let mint_data = mint_account.data.downcast_mut::<CypherMintData>().unwrap();
                mint_data.supply -= params.amount;
                self.state_manager.update_account(params.mint, mint_account);
            }
            ParsedInstruction::CypherMint { params } => {
                self.storage
                    .store_instruction(instruction.clone(), slot, tx_signature)
                    .await?;
                let mint_account = self
                    .state_manager
                    .get_account(&params.mint)
                    .ok_or(Box::new(ProcessorError::InvalidAccount))?;
                let mint_data = mint_account.data.downcast_mut::<CypherMintData>().unwrap();
                mint_data.supply += params.amount;
                self.state_manager.update_account(params.mint, mint_account);
            }
            ParsedInstruction::CypherBurn { params } => {
                self.storage
                    .store_instruction(instruction.clone(), slot, tx_signature)
                    .await?;
                let mint_account = self
                    .state_manager
                    .get_account(&params.mint)
                    .ok_or(Box::new(ProcessorError::InvalidAccount))?;
                let mint_data = mint_account.data.downcast_mut::<CypherMintData>().unwrap();
                mint_data.supply -= params.amount;
                self.state_manager.update_account(params.mint, mint_account);
            }
            ParsedInstruction::CreateAssociatedCypherAccount {
                funding_account,
                associated_account,
                wallet_account,
                cypher_mint,
            } => {
                self.storage
                    .store_instruction(instruction.clone(), slot, tx_signature)
                    .await?;
                self.state_manager.update_account(
                    funding_account,
                    ParsedAccount::Unknown {
                        pubkey: funding_account,
                        owner: wallet_account,
                        data: vec![],
                    },
                );
                self.state_manager.update_account(
                    associated_account,
                    ParsedAccount::Unknown {
                        pubkey: associated_account,
                        owner: wallet_account,
                        data: vec![],
                    },
                );
                self.state_manager.update_account(
                    cypher_mint,
                    ParsedAccount::CypherMint {
                        pubkey: cypher_mint,
                        data: CypherMintData {
                            supply: 0,
                            decimals: 9,
                            mint_authority: funding_account,
                            freeze_authority: None,
                        },
                    },
                );
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::storage::Storage;
    use solana_sdk::pubkey::Pubkey;
    use std::time::Duration;

    #[tokio::test]
    async fn test_processor() {
        let config = Config::load().unwrap();
        let storage = Storage::new(&config).await.unwrap();
        let ipfs_storage = IpfsStorage::new(&config.ipfs_api_url);
        let processor = Processor::new(storage, ipfs_storage);

        let pubkey = Pubkey::new_unique();
        let data = vec![0, 1, 2, 3];
        let owner = Pubkey::new_unique();

        processor
            .process_account_update(pubkey, data, owner, 0)
            .await
            .unwrap();

        let account = processor.state_manager.get_account(&pubkey).unwrap();
        assert_eq!(account.pubkey, pubkey);
        assert_eq!(account.data.len(), 4);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error("Invalid account")]
    InvalidAccount,
}

impl From<Box<dyn std::error::Error>> for ProcessorError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        ProcessorError::InvalidAccount
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::storage::Storage;
    use solana_sdk::pubkey::Pubkey;
    use std::time::Duration;

    #[tokio::test]
    async fn test_processor() {
        let config = Config::load().unwrap();
        let storage = Storage::new(&config).await.unwrap();
        let ipfs_storage = IpfsStorage::new(&config.ipfs_api_url);
        let processor = Processor::new(storage, ipfs_storage);

        let pubkey = Pubkey::new_unique();
        let data = vec![0, 1, 2, 3];
        let owner = Pubkey::new_unique();

        processor
            .process_account_update(pubkey, data, owner, 0)
            .await
            .unwrap();

        let account = processor.state_manager.get_account(&pubkey).unwrap();
        assert_eq!(account.pubkey, pubkey);
        assert_eq!(account.data.len(), 4);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error("Invalid account")]
    InvalidAccount,
}

impl From<Box<dyn std::error::Error>> for ProcessorError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        ProcessorError::InvalidAccount
    }
}
