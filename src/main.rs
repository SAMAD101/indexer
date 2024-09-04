mod cli;
mod db;
mod indexer;
mod models;
mod schema;

use anyhow::Result;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");

    let config = cli::parse_args();
    let pool = db::establish_connection(&database_url)?;

    indexer::processor::run(config, pool, &rpc_url).await
}