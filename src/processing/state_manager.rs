use crate::processing::account_parser::ParsedAccount;
use dashmap::DashMap;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

pub struct StateManager {
    accounts: DashMap<Pubkey, ParsedAccount>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            accounts: DashMap::new(),
        }
    }

    pub fn update_account(&self, pubkey: Pubkey, account: ParsedAccount) {
        self.accounts.insert(pubkey, account);
    }

    pub fn get_account(&self, pubkey: &Pubkey) -> Option<ParsedAccount> {
        self.accounts.get(pubkey).map(|a| a.value().clone())
    }

    pub fn remove_account(&self, pubkey: &Pubkey) {
        self.accounts.remove(pubkey);
    }

    pub fn get_all_accounts(&self) -> HashMap<Pubkey, ParsedAccount> {
        self.accounts
            .iter()
            .map(|entry| (*entry.key(), entry.value().clone()))
            .collect()
    }

    pub fn get_accounts_by_type(&self, account_type: &str) -> Vec<(Pubkey, ParsedAccount)> {
        self.accounts
            .iter()
            .filter_map(|entry| match entry.value() {
                ParsedAccount::CypherMint { .. } if account_type == "CypherMint" => {
                    Some((*entry.key(), entry.value().clone()))
                }
                ParsedAccount::CypherToken { .. } if account_type == "CypherToken" => {
                    Some((*entry.key(), entry.value().clone()))
                }
                ParsedAccount::CypherMetadata { .. } if account_type == "CypherMetadata" => {
                    Some((*entry.key(), entry.value().clone()))
                }
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processing::account_parser::{CypherMetadata, CypherMintData, CypherTokenData};

    #[test]
    fn test_state_manager() {
        let state_manager = StateManager::new();

        let pubkey1 = Pubkey::new_unique();
        let pubkey2 = Pubkey::new_unique();

        let account1 = ParsedAccount::CypherMint {
            pubkey: pubkey1,
            data: CypherMintData {
                supply: 1000,
                decimals: 9,
                mint_authority: Pubkey::new_unique(),
                freeze_authority: None,
            },
        };

        let account2 = ParsedAccount::CypherToken {
            pubkey: pubkey2,
            data: CypherTokenData {
                mint: pubkey1,
                owner: Pubkey::new_unique(),
                amount: 500,
                delegate: None,
                state: 1,
                is_native: None,
                delegated_amount: 0,
                close_authority: None,
            },
        };

        state_manager.update_account(pubkey1, account1.clone());
        state_manager.update_account(pubkey2, account2.clone());

        assert_eq!(state_manager.get_account(&pubkey1), Some(account1));
        assert_eq!(state_manager.get_account(&pubkey2), Some(account2));

        let mint_accounts = state_manager.get_accounts_by_type("CypherMint");
        assert_eq!(mint_accounts.len(), 1);
        assert_eq!(mint_accounts[0].0, pubkey1);

        let token_accounts = state_manager.get_accounts_by_type("CypherToken");
        assert_eq!(token_accounts.len(), 1);
        assert_eq!(token_accounts[0].0, pubkey2);

        state_manager.remove_account(&pubkey1);
        assert_eq!(state_manager.get_account(&pubkey1), None);

        let all_accounts = state_manager.get_all_accounts();
        assert_eq!(all_accounts.len(), 1);
        assert!(all_accounts.contains_key(&pubkey2));
    }
}
