use crate::models::account::Account;
use crate::models::transaction::Transaction;
use crate::schema::{accounts, transactions};
use anyhow::Result;
use diesel::prelude::*;
use super::DbPool;

pub fn insert_account(pool: &DbPool, account: &Account) -> Result<()> {
    let conn = &mut pool.get()?;
    diesel::insert_into(accounts::table)
        .values(account)
        .on_conflict(accounts::pubkey)
        .do_update()
        .set(account)
        .execute(conn)?;
    Ok(())
}

pub fn insert_transaction(pool: &DbPool, transaction: &Transaction) -> Result<()> {
    let conn = &mut pool.get()?;
    diesel::insert_into(transactions::table)
        .values(transaction)
        .on_conflict(transactions::signature)
        .do_nothing()
        .execute(conn)?;
    Ok(())
}