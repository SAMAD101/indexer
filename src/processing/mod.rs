use crate::storage::Storage;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{EncodedConfirmedTransaction, UiConfirmedBlock};

pub mod account_parser;
pub mod instruction_parser;
pub mod event_parser;
pub mod state_manager;

use account_parser::AccountParser;
use instruction_parser::InstructionParser;
use event_parser::EventParser;
use state_manager::StateManager;

#[derive(Clone)]
pub struct Processor {
    storage: Storage,
    account_parser: AccountParser,
    instruction_parser: InstructionParser,
    event_parser: EventParser,
    state_manager: StateManager,
}

impl Processor {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
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

        for (index, instruction) in transaction.transaction.message.instructions.iter().enumerate() {
            let program_id = transaction.transaction.message.account_keys[instruction.program_id_index as usize];
            let parsed_instruction = self.instruction_parser.parse_instruction(&program_id, instruction)?;
            self.storage.store_instruction(parsed_instruction, slot, &signature).await?;
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
                        let parsed_account = self.account_parser.parse_account(pubkey, account_data, &Pubkey::default())?;
                        self.state_manager.update_account(*pubkey, parsed_account.clone());
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
        self.state_manager.update_account(pubkey, parsed_account.clone());
        self.storage.store_account(parsed_account, slot).await?;
        Ok(())
    }
}