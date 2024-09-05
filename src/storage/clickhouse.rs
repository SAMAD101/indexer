use clickhouse::{Client, Row};
use crate::processing::{ParsedAccount, ParsedInstruction, ParsedEvent};

pub struct ClickhouseStorage {
    client: Client,
}

impl ClickhouseStorage {
    pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::default()
            .with_url(url)
            .with_database("cypher_indexer")
            .await?;
        Ok(Self { client })
    }

    pub async fn store_account(&self, account: ParsedAccount, slot: u64) -> Result<(), Box<dyn std::error::Error>> {
        let query = "INSERT INTO accounts (pubkey, owner, slot, account_type, data) VALUES (?, ?, ?, ?, ?)";
        let mut rows = Vec::new();
        match account {
            ParsedAccount::Token { /* fields */ } => {
                // Implement token account storage
            },
            ParsedAccount::Mint { /* fields */ } => {
                // Implement mint account storage
            },
            ParsedAccount::Multisig { /* fields */ } => {
                // Implement multisig account storage
            },
            ParsedAccount::Program { pubkey, owner } => {
                rows.push(Row::new(vec![
                    pubkey.to_string().into(),
                    owner.to_string().into(),
                    slot.into(),
                    "program".into(),
                    Vec::<u8>::new().into(),
                ]));
            },
            ParsedAccount::Unknown { pubkey, owner, data } => {
                rows.push(Row::new(vec![
                    pubkey.to_string().into(),
                    owner.to_string().into(),
                    slot.into(),
                    "unknown".into(),
                    data.into(),
                ]));
            },
        }
        self.client.insert(query, rows).await?;
        Ok(())
    }

    pub async fn store_instruction(&self, instruction: ParsedInstruction, slot: u64, tx_signature: &str) -> Result<(), Box<dyn std::error::Error>> {
        let query = "INSERT INTO instructions (slot, tx_signature, program_id, instruction_type, data) VALUES (?, ?, ?, ?, ?)";
        let mut rows = Vec::new();
        match instruction {
            ParsedInstruction::Token { /* fields */ } => {
                // Implement token instruction storage
            },
            ParsedInstruction::AssociatedToken { /* fields */ } => {
                // Implement associated token instruction storage
            },
            ParsedInstruction::Unknown { program_id, data } => {
                rows.push(Row::new(vec![
                    slot.into(),
                    tx_signature.into(),
                    program_id.to_string().into(),
                    "unknown".into(),
                    data.into(),
                ]));
            },
        }
        self.client.insert(query, rows).await?;
        Ok(())
    }

    pub async fn store_event(&self, event: ParsedEvent, slot: u64, tx_signature: &str) -> Result<(), Box<dyn std::error::Error>> {
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
            },
            ParsedEvent::Plain(text) => {
                rows.push(Row::new(vec![
                    slot.into(),
                    tx_signature.into(),
                    "plain".into(),
                    text.into(),
                ]));
            },
        }
        self.client.insert(query, rows).await?;
        Ok(())
    }
}