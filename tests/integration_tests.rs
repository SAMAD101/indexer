use cypher_indexer::{
    config::Config,
    storage::Storage,
    processing::Processor,
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_transaction_status::{
    EncodedConfirmedTransaction,
    UiTransactionStatusMeta,
    TransactionStatusMeta,
    InnerInstructions,
    UiConfirmedBlock,
};

#[tokio::test]
async fn test_process_transaction() {
    let config = Config::load().expect("Failed to load config");
    let storage = Storage::new(&config).await.expect("Failed to initialize storage");
    let processor = Processor::new(storage);

    let payer = Keypair::new();
    let to = Pubkey::new_unique();
    let lamports = 1_000_000;
    let recent_blockhash = solana_sdk::hash::Hash::default();

    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(&payer.pubkey(), &to, lamports)],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let encoded_tx = EncodedConfirmedTransaction {
        slot: 12345,
        transaction: tx.encode(),
        meta: Some(TransactionStatusMeta {
            status: Ok(()),
            fee: 5000,
            pre_balances: vec![10_000_000, 0],
            post_balances: vec![8_995_000, 1_000_000],
            inner_instructions: Some(vec![]),
            log_messages: Some(vec!["Program 11111111111111111111111111111111 invoke [1]".to_string()]),
            pre_token_balances: Some(vec![]),
            post_token_balances: Some(vec![]),
            rewards: Some(vec![]),
        }),
    };

    let result = processor.process_transaction(encoded_tx, 12345).await;
    assert!(result.is_ok());

    // Add assertions to check if the transaction was processed correctly
    // For example, check if the transaction is stored in the database
    // let stored_tx = storage.get_transaction(&tx.signatures[0].to_string()).await.unwrap();
    // assert_eq!(stored_tx.slot, 12345);
}

#[tokio::test]
async fn test_process_account_update() {
    let config = Config::load().expect("Failed to load config");
    let storage = Storage::new(&config).await.expect("Failed to initialize storage");
    let processor = Processor::new(storage);

    let pubkey = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let data = vec![0, 1, 2, 3, 4];
    let slot = 12345;

    let result = processor.process_account_update(pubkey, data.clone(), owner, slot).await;
    assert!(result.is_ok());

    // Add assertions to check if the account update was processed correctly
    // For example, check if the account data is stored in the database
    // let stored_account = storage.get_account(&pubkey.to_string()).await.unwrap();
    // assert_eq!(stored_account.data, data);
    // assert_eq!(stored_account.owner, owner.to_string());
    // assert_eq!(stored_account.slot, slot);
}

#[tokio::test]
async fn test_process_block() {
    let config = Config::load().expect("Failed to load config");
    let storage = Storage::new(&config).await.expect("Failed to initialize storage");
    let processor = Processor::new(storage);

    // Create a mock block
    let block = UiConfirmedBlock {
        previous_blockhash: "11111111111111111111111111111111".to_string(),
        blockhash: "22222222222222222222222222222222".to_string(),
        parent_slot: 12344,
        transactions: vec![
            Some(EncodedConfirmedTransaction {
                slot: 12345,
                transaction: Transaction::default().encode(),
                meta: Some(TransactionStatusMeta::default()),
            }),
        ],
        rewards: Some(vec![]),
        block_time: Some(1234567890),
        block_height: Some(10000),
    };

    let result = processor.process_block(block, 12345).await;
    assert!(result.is_ok());

    // Add assertions to check if the block was processed correctly
    // For example, check if all transactions in the block are stored in the database
    // let stored_tx = storage.get_transaction(&Transaction::default().signatures[0].to_string()).await.unwrap();
    // assert_eq!(stored_tx.slot, 12345);
}