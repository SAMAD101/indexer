use async_graphql::{Context, Object, Schema, EmptyMutation, EmptySubscription, SimpleObject, ID};
use crate::storage::Storage;

struct Query;

#[derive(SimpleObject)]
struct Account {
    pubkey: ID,
    owner: ID,
    lamports: i64,
    data: String,
    executable: bool,
}

#[derive(SimpleObject)]
struct Transaction {
    signature: ID,
    slot: i64,
    success: bool,
    fee: i64,
    logs: Vec<String>,
}

#[Object]
impl Query {
    async fn get_account(&self, ctx: &Context<'_>, pubkey: ID) -> async_graphql::Result<Option<Account>> {
        let storage = ctx.data::<Storage>()?;
        storage.get_account(pubkey.as_str()).await.map_err(|e| e.into())
    }

    async fn get_transaction(&self, ctx: &Context<'_>, signature: ID) -> async_graphql::Result<Option<Transaction>> {
        let storage = ctx.data::<Storage>()?;
        storage.get_transaction(signature.as_str()).await.map_err(|e| e.into())
    }

    async fn get_transactions_by_account(&self, ctx: &Context<'_>, pubkey: ID, limit: i32) -> async_graphql::Result<Vec<Transaction>> {
        let storage = ctx.data::<Storage>()?;
        storage.get_transactions_by_account(pubkey.as_str(), limit).await.map_err(|e| e.into())
    }
}

pub type CypherIndexerSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema(storage: Storage) -> CypherIndexerSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(storage)
        .finish()
}