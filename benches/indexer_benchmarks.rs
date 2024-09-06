use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cypher_indexer::{
    config::Config,
    processing::{
        account_parser::AccountParser, event_parser::EventParser,
        instruction_parser::InstructionParser, Processor,
    },
    storage::{ipfs::IpfsStorage, Storage},
};
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey, transaction::Transaction};
use solana_transaction_status::{EncodedConfirmedTransaction, UiConfirmedBlock};
use std::collections::HashMap;
use tokio::runtime::Runtime;

fn create_mock_config() -> Config {
    Config {
        solana_rpc_url: "https://api.devnet.solana.com".to_string(),
        clickhouse_url: "http://localhost:8123".to_string(),
        scylla_nodes: vec!["127.0.0.1:9042".to_string()],
        redis_url: "redis://localhost:6379".to_string(),
        ipfs_api_url: "http://localhost:5001".to_string(),
        wasm_module_path: "./wasm/cypher_indexer.wasm".to_string(),
        rpc_poll_interval: 1,
        websocket_url: "wss://api.devnet.solana.com".to_string(),
        geyser_plugin_config: GeyserPluginConfig {
            libpath: "/path/to/mock/libsolana_geyser_plugin.so".to_string(),
            accounts_selector: AccountsSelector {
                owners: vec!["CyphrkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string()],
            },
        },
        wasm_modules: Some(HashMap::from([(
            "test_module".to_string(),
            "./tests/test_module.wasm".to_string(),
        )])),
        wasm_memory_limit: Some(1024 * 1024),
        wasm_execution_timeout: Some(5),
    }
}

fn create_mock_storage() -> Storage {
    let config = create_mock_config();
    let rt = Runtime::new().unwrap();
    rt.block_on(async { Storage::new(&config).await.unwrap() })
}

fn create_mock_ipfs_storage() -> IpfsStorage {
    IpfsStorage::new("http://localhost:5001")
}

fn benchmark_processor_new(c: &mut Criterion) {
    let storage = create_mock_storage();
    let ipfs_storage = create_mock_ipfs_storage();

    c.bench_function("Processor::new", |b| {
        b.iter(|| Processor::new(black_box(storage.clone()), black_box(ipfs_storage.clone())))
    });
}

fn benchmark_process_transaction(c: &mut Criterion) {
    let storage = create_mock_storage();
    let ipfs_storage = create_mock_ipfs_storage();
    let processor = Processor::new(storage, ipfs_storage);

    let mock_transaction = EncodedConfirmedTransaction {
        slot: 1000,
        transaction: Transaction::default(),
        meta: None,
    };

    c.bench_function("Processor::process_transaction", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                processor
                    .process_transaction(black_box(mock_transaction.clone()), black_box(1000))
                    .await
            })
        })
    });
}

fn benchmark_process_block(c: &mut Criterion) {
    let storage = create_mock_storage();
    let ipfs_storage = create_mock_ipfs_storage();
    let processor = Processor::new(storage, ipfs_storage);

    let mock_block = UiConfirmedBlock {
        previous_blockhash: "11111111111111111111111111111111".to_string(),
        blockhash: "22222222222222222222222222222222".to_string(),
        parent_slot: 999,
        transactions: vec![Some(EncodedConfirmedTransaction {
            slot: 1000,
            transaction: Transaction::default(),
            meta: None,
        })],
        rewards: None,
        block_time: Some(1623456789),
        block_height: Some(1000),
    };

    c.bench_function("Processor::process_block", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                processor
                    .process_block(black_box(mock_block.clone()), black_box(1000))
                    .await
            })
        })
    });
}

fn benchmark_account_parser(c: &mut Criterion) {
    let account_parser = AccountParser::new();
    let mock_pubkey = Pubkey::new_unique();
    let mock_data = vec![0u8; 100];
    let mock_owner = Pubkey::new_unique();

    c.bench_function("AccountParser::parse_account", |b| {
        b.iter(|| {
            account_parser.parse_account(
                black_box(&mock_pubkey),
                black_box(&mock_data),
                black_box(&mock_owner),
            )
        })
    });
}

fn benchmark_instruction_parser(c: &mut Criterion) {
    let instruction_parser = InstructionParser::new();
    let mock_program_id = Pubkey::new_unique();
    let mock_instruction = CompiledInstruction {
        program_id_index: 0,
        accounts: vec![0, 1, 2],
        data: vec![0u8; 32],
    };

    c.bench_function("InstructionParser::parse_instruction", |b| {
        b.iter(|| {
            instruction_parser
                .parse_instruction(black_box(&mock_program_id), black_box(&mock_instruction))
        })
    });
}

fn benchmark_event_parser(c: &mut Criterion) {
    let event_parser = EventParser::new();
    let mock_logs = vec![
        "Program log: {\"type\":\"cypher_transfer\",\"from\":\"ABC\",\"to\":\"XYZ\",\"amount\":1000}".to_string(),
        "Program log: Some other log".to_string(),
    ];

    c.bench_function("EventParser::parse_logs", |b| {
        b.iter(|| event_parser.parse_logs(black_box(&mock_logs)))
    });
}

criterion_group!(
    benches,
    benchmark_processor_new,
    benchmark_process_transaction,
    benchmark_process_block,
    benchmark_account_parser,
    benchmark_instruction_parser,
    benchmark_event_parser
);
criterion_main!(benches);

#[derive(Clone)]
struct GeyserPluginConfig {
    libpath: String,
    accounts_selector: AccountsSelector,
}

#[derive(Clone)]
struct AccountsSelector {
    owners: Vec<String>,
}
