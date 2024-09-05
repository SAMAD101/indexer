use solana_sdk::pubkey::Pubkey;
use solana_account_decoder::UiAccountEncoding;
use solana_transaction_status::{UiTransactionEncoding, EncodedConfirmedTransaction};
use bs58;

pub fn is_program_account(account_data: &[u8]) -> bool {
    account_data.len() >= 4 && account_data[0..4] == [0, 0, 0, 0]
}

pub fn get_account_type(account_data: &[u8]) -> Option<u8> {
    if account_data.len() >= 5 {
        Some(account_data[4])
    } else {
        None
    }
}

pub fn decode_account_data(account_data: &[u8], encoding: UiAccountEncoding) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match encoding {
        UiAccountEncoding::Base58 => Ok(bs58::decode(account_data).into_vec()?),
        UiAccountEncoding::Base64 => Ok(base64::decode(account_data)?),
        UiAccountEncoding::Base64Zstd => {
            let decoded = base64::decode(account_data)?;
            Ok(zstd::decode_all(&decoded[..])?
)
        },
        _ => Err("Unsupported account data encoding".into()),
    }
}

pub fn pubkey_from_str(s: &str) -> Result<Pubkey, Box<dyn std::error::Error>> {
    Ok(s.parse::<Pubkey>()?)
}

pub fn encode_transaction_for_rpc(
    transaction: &EncodedConfirmedTransaction,
) -> Result<String, Box<dyn std::error::Error>> {
    match transaction.transaction.encode(UiTransactionEncoding::Base64) {
        Some(encoded) => Ok(encoded),
        None => Err("Failed to encode transaction".into()),
    }
}

pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1_000_000_000.0) as u64
}

pub fn shorten_pubkey(pubkey: &Pubkey) -> String {
    let s = pubkey.to_string();
    format!("{}...{}", &s[..4], &s[s.len() - 4..])
}

pub fn is_system_program(program_id: &Pubkey) -> bool {
    program_id == &solana_sdk::system_program::id()
}

pub fn is_spl_token_program(program_id: &Pubkey) -> bool {
    program_id == &spl_token::id()
}

pub fn extract_program_id(instruction_data: &[u8]) -> Option<Pubkey> {
    if instruction_data.len() >= 32 {
        Some(Pubkey::new(&instruction_data[..32]))
    } else {
        None
    }
}

pub fn decode_instruction_data(instruction_data: &[u8]) -> Vec<u8> {
    if instruction_data.len() > 32 {
        instruction_data[32..].to_vec()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_program_account() {
        assert!(is_program_account(&[0, 0, 0, 0, 1, 2, 3]));
        assert!(!is_program_account(&[1, 0, 0, 0, 1, 2, 3]));
        assert!(!is_program_account(&[0, 0, 0]));
    }

    #[test]
    fn test_get_account_type() {
        assert_eq!(get_account_type(&[0, 0, 0, 0, 1, 2, 3]), Some(1));
        assert_eq!(get_account_type(&[0, 0, 0, 0]), None);
    }

    #[test]
    fn test_lamports_to_sol() {
        assert_eq!(lamports_to_sol(1_000_000_000), 1.0);
        assert_eq!(lamports_to_sol(500_000_000), 0.5);
    }

    #[test]
    fn test_sol_to_lamports() {
        assert_eq!(sol_to_lamports(1.0), 1_000_000_000);
        assert_eq!(sol_to_lamports(0.5), 500_000_000);
    }

    #[test]
    fn test_shorten_pubkey() {
        let pubkey = Pubkey::new_unique();
        let shortened = shorten_pubkey(&pubkey);
        assert_eq!(shortened.len(), 11);
        assert_eq!(&shortened[4..7], "...");
    }
}