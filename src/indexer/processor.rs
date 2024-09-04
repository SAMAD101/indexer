use crate::cli::Args;
use crate::db::{DbPool, operations};
use crate::models::account::Account;
use crate::models::transaction::Transaction;
use anyhow::Result;
use chrono::Utc;
use log::{info, error};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedTransaction, UiMessage};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use solana_transaction_status::parse_accounts::ParsedAccount;

pub async fn run(config: Args, db_pool: DbPool, rpc_url: &str) -> Result<()> {
    let rpc_client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    info!("Starting indexer...");

    let mut slot = rpc_client.get_slot()?;

    loop {
        let signatures = rpc_client.get_signatures_for_address(&Pubkey::from_str(&config.program_id)?)?;

        if signatures.is_empty() {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            continue;
        }

        for sig_info in signatures.iter().take(config.batch_size as usize) {
            let signature = Signature::from_str(&sig_info.signature)?;
            let tx_with_meta = rpc_client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            )?;

            let transaction = Transaction {
                signature: sig_info.signature.clone(),
                slot: tx_with_meta.slot as i64,
                err: tx_with_meta.transaction.meta.and_then(|m| m.err.map(|e| e.to_string())),
                memo: None,
                block_time: tx_with_meta.block_time.map(|t| t as i64),
                created_at: Utc::now().naive_utc(),
            };

            if let Err(e) = operations::insert_transaction(&db_pool, &transaction) {
                error!("Error inserting transaction: {:?}", e);
            }

            match tx_with_meta.transaction.transaction {
                EncodedTransaction::Json(json_transaction) => {
                    let account_keys = match &json_transaction.message {
                        UiMessage::Parsed(parsed_message) => {
                            parsed_message.account_keys.iter()
                                .filter_map(|account| {
                                    match account {
                                        ParsedAccount::Parsed { pubkey: pubkey_str, .. } => Some(pubkey_str.to_string()),
                                        ParsedAccount::Raw(pubkey) => Some(pubkey.to_string()),
                                    }
                                })
                                .collect::<Vec<String>>()
                        },
                        UiMessage::Raw(raw_message) => {
                            raw_message.account_keys.clone()
                        },
                    };

                    for pubkey_str in account_keys {
                        if let Ok(pubkey) = Pubkey::from_str(&pubkey_str) {
                            if let Ok(account_info) = rpc_client.get_account(&pubkey) {
                                let account = Account {
                                    pubkey: pubkey.to_string(),
                                    lamports: account_info.lamports as i64,
                                    owner: account_info.owner.to_string(),
                                    executable: account_info.executable,
                                    rent_epoch: account_info.rent_epoch as i64,
                                    data: account_info.data.clone(),
                                    updated_at: Utc::now().naive_utc(),
                                };

                                if let Err(e) = operations::insert_account(&db_pool, &account) {
                                    error!("Error inserting account: {:?}", e);
                                }
                            }
                        }
                    }
                },
                _ => {
                }
            }

            slot = signatures.last().map(|sig| sig.slot).unwrap_or(slot);
            info!("Processed {} transactions, current slot: {}", signatures.len(), slot);
        }
    }
}