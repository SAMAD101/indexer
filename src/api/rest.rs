use warp::{Filter, Rejection, Reply};
use crate::storage::Storage;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
}

#[derive(Deserialize)]
struct AccountQuery {
    pubkey: String,
}

#[derive(Deserialize)]
struct TransactionQuery {
    signature: String,
}

pub fn routes(
    storage: Storage,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let storage = warp::any().map(move || storage.clone());

    let account = warp::path("account")
        .and(warp::get())
        .and(warp::query::<AccountQuery>())
        .and(storage.clone())
        .and_then(get_account);

    let transaction = warp::path("transaction")
        .and(warp::get())
        .and(warp::query::<TransactionQuery>())
        .and(storage.clone())
        .and_then(get_transaction);

    account.or(transaction)
}

async fn get_account(query: AccountQuery, storage: Storage) -> Result<impl Reply, Rejection> {
    let account = storage.get_account(&query.pubkey).await.map_err(|e| warp::reject::custom(e))?;
    Ok(warp::reply::json(&ApiResponse {
        status: "success".to_string(),
        data: account,
    }))
}

async fn get_transaction(query: TransactionQuery, storage: Storage) -> Result<impl Reply, Rejection> {
    let transaction = storage.get_transaction(&query.signature).await.map_err(|e| warp::reject::custom(e))?;
    Ok(warp::reply::json(&ApiResponse {
        status: "success".to_string(),
        data: transaction,
    }))
}