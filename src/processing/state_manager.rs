use dashmap::DashMap;
use solana_sdk::pubkey::Pubkey;
use crate::processing::ParsedAccount;

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
        self.accounts.get(pubkey).map(|a| a.clone())
    }

    pub fn remove_account(&self, pubkey: &Pubkey) {
        self.accounts.remove(pubkey);
    }
}