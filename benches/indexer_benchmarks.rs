use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
    TransactionStatusMeta,
};

fn benchmark_process_transaction(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let config = Config::load().unwrap();
        let storage = Storage::new(&config).await.unwrap();
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
            meta: Some(TransactionStatusMeta::default()),
        };

        c.bench_function("process_transaction", |b| {
            b.to_async(&rt).iter(|| async {
                black_box(processor.process_transaction(encoded_tx.clone(), 12345).await)
            })
        });
    });
}

fn benchmark_process_account_update(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let config = Config::load().unwrap();
        let storage = Storage::new(&config).await.unwrap();
        let processor = Processor::new(storage);

        let pubkey = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let data = vec![0, 1, 2, 3, 4];
        let slot = 12345;

        c.bench_function("process_account_update", |b| {
            b.to_async(&rt).iter(|| async {
                black_box(processor.process_account_update(pubkey, data.clone(), owner, slot).await)
            })
        });
    });
}

criterion_group!(benches, benchmark_process_transaction, benchmark_process_account_update);
criterion_main!(benches);