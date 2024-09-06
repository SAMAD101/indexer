use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::instruction::{AccountMeta, CompiledInstruction};
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstructionParseError {
    #[error("Failed to deserialize instruction data: {0}")]
    DeserializationError(#[from] std::io::Error),
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(u8),
    #[error("Invalid account index: {0}")]
    InvalidAccountIndex(usize),
}

pub struct InstructionParser;

impl InstructionParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_instruction(
        &self,
        program_id: &Pubkey,
        instruction: &CompiledInstruction,
    ) -> Result<ParsedInstruction, InstructionParseError> {
        match program_id.to_string().as_str() {
            "CyphrkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => {
                self.parse_cypher_instruction(instruction)
            }
            "ACyphrGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" => {
                self.parse_associated_cypher_instruction(instruction)
            }
            _ => self.parse_unknown_instruction(program_id, instruction),
        }
    }

    fn parse_cypher_instruction(
        &self,
        instruction: &CompiledInstruction,
    ) -> Result<ParsedInstruction, InstructionParseError> {
        let instruction_type: u8 = instruction.data[0];
        match instruction_type {
            0 => self.parse_cypher_initialize(instruction),
            1 => self.parse_cypher_transfer(instruction),
            2 => self.parse_cypher_mint(instruction),
            3 => self.parse_cypher_burn(instruction),
            _ => Err(InstructionParseError::UnknownInstruction(instruction_type)),
        }
    }

    fn parse_associated_cypher_instruction(
        &self,
        instruction: &CompiledInstruction,
    ) -> Result<ParsedInstruction, InstructionParseError> {
        let instruction_type: u8 = instruction.data[0];
        match instruction_type {
            0 => self.parse_create_associated_cypher_account(instruction),
            _ => Err(InstructionParseError::UnknownInstruction(instruction_type)),
        }
    }

    fn parse_unknown_instruction(
        &self,
        program_id: &Pubkey,
        instruction: &CompiledInstruction,
    ) -> Result<ParsedInstruction, InstructionParseError> {
        Ok(ParsedInstruction::Unknown {
            program_id: *program_id,
            data: instruction.data.clone(),
        })
    }

    fn parse_cypher_initialize(
        &self,
        instruction: &CompiledInstruction,
    ) -> Result<ParsedInstruction, InstructionParseError> {
        let params = CypherInitializeParams::try_from_slice(&instruction.data[1..])?;
        Ok(ParsedInstruction::CypherInitialize { params })
    }

    fn parse_cypher_transfer(
        &self,
        instruction: &CompiledInstruction,
    ) -> Result<ParsedInstruction, InstructionParseError> {
        let params = CypherTransferParams::try_from_slice(&instruction.data[1..])?;
        Ok(ParsedInstruction::CypherTransfer { params })
    }

    fn parse_cypher_mint(
        &self,
        instruction: &CompiledInstruction,
    ) -> Result<ParsedInstruction, InstructionParseError> {
        let params = CypherMintParams::try_from_slice(&instruction.data[1..])?;
        Ok(ParsedInstruction::CypherMint { params })
    }

    fn parse_cypher_burn(
        &self,
        instruction: &CompiledInstruction,
    ) -> Result<ParsedInstruction, InstructionParseError> {
        let params = CypherBurnParams::try_from_slice(&instruction.data[1..])?;
        Ok(ParsedInstruction::CypherBurn { params })
    }

    fn parse_create_associated_cypher_account(
        &self,
        instruction: &CompiledInstruction,
    ) -> Result<ParsedInstruction, InstructionParseError> {
        Ok(ParsedInstruction::CreateAssociatedCypherAccount {
            funding_account: self.get_account_pubkey(instruction, 0)?,
            associated_account: self.get_account_pubkey(instruction, 1)?,
            wallet_account: self.get_account_pubkey(instruction, 2)?,
            cypher_mint: self.get_account_pubkey(instruction, 3)?,
        })
    }

    fn get_account_pubkey(
        &self,
        instruction: &CompiledInstruction,
        index: usize,
    ) -> Result<Pubkey, InstructionParseError> {
        instruction
            .accounts
            .get(index)
            .ok_or(InstructionParseError::InvalidAccountIndex(index))
            .map(|&account_index| Pubkey::new(&[account_index]))
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CypherInitializeParams {
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CypherTransferParams {
    pub amount: u64,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CypherMintParams {
    pub amount: u64,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CypherBurnParams {
    pub amount: u64,
}

#[derive(Debug)]
pub enum ParsedInstruction {
    CypherInitialize {
        params: CypherInitializeParams,
    },
    CypherTransfer {
        params: CypherTransferParams,
    },
    CypherMint {
        params: CypherMintParams,
    },
    CypherBurn {
        params: CypherBurnParams,
    },
    CreateAssociatedCypherAccount {
        funding_account: Pubkey,
        associated_account: Pubkey,
        wallet_account: Pubkey,
        cypher_mint: Pubkey,
    },
    Unknown {
        program_id: Pubkey,
        data: Vec<u8>,
    },
}
