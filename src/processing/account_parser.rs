use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountParseError {
    #[error("Failed to deserialize account data: {0}")]
    DeserializationError(#[from] std::io::Error),
    #[error("Invalid account type: {0}")]
    InvalidAccountType(u8),
    #[error("Invalid account data length")]
    InvalidDataLength,
}

pub struct AccountParser;

impl AccountParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_account(
        &self,
        pubkey: &Pubkey,
        data: &[u8],
        owner: &Pubkey,
    ) -> Result<ParsedAccount, AccountParseError> {
        if data.len() < 1 {
            return Err(AccountParseError::InvalidDataLength);
        }

        let account_type = data[0];
        match account_type {
            0 => self.parse_cypher_mint_account(pubkey, data),
            1 => self.parse_cypher_token_account(pubkey, data),
            2 => self.parse_cypher_metadata_account(pubkey, data),
            _ => self.parse_unknown_account(pubkey, data, owner),
        }
    }

    fn parse_cypher_mint_account(
        &self,
        pubkey: &Pubkey,
        data: &[u8],
    ) -> Result<ParsedAccount, AccountParseError> {
        let mint_data = CypherMintData::try_from_slice(&data[1..])?;
        Ok(ParsedAccount::CypherMint {
            pubkey: *pubkey,
            data: mint_data,
        })
    }

    fn parse_cypher_token_account(
        &self,
        pubkey: &Pubkey,
        data: &[u8],
    ) -> Result<ParsedAccount, AccountParseError> {
        let token_data = CypherTokenData::try_from_slice(&data[1..])?;
        Ok(ParsedAccount::CypherToken {
            pubkey: *pubkey,
            data: token_data,
        })
    }

    fn parse_cypher_metadata_account(
        &self,
        pubkey: &Pubkey,
        data: &[u8],
    ) -> Result<ParsedAccount, AccountParseError> {
        let metadata = CypherMetadata::try_from_slice(&data[1..])?;
        Ok(ParsedAccount::CypherMetadata {
            pubkey: *pubkey,
            data: metadata,
        })
    }

    fn parse_unknown_account(
        &self,
        pubkey: &Pubkey,
        data: &[u8],
        owner: &Pubkey,
    ) -> Result<ParsedAccount, AccountParseError> {
        Ok(ParsedAccount::Unknown {
            pubkey: *pubkey,
            owner: *owner,
            data: data.to_vec(),
        })
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CypherMintData {
    pub supply: u64,
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CypherTokenData {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub delegate: Option<Pubkey>,
    pub state: u8,
    pub is_native: Option<u64>,
    pub delegated_amount: u64,
    pub close_authority: Option<Pubkey>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CypherMetadata {
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug)]
pub enum ParsedAccount {
    CypherMint {
        pubkey: Pubkey,
        data: CypherMintData,
    },
    CypherToken {
        pubkey: Pubkey,
        data: CypherTokenData,
    },
    CypherMetadata {
        pubkey: Pubkey,
        data: CypherMetadata,
    },
    Unknown {
        pubkey: Pubkey,
        owner: Pubkey,
        data: Vec<u8>,
    },
}
