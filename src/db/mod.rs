pub mod operations;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use anyhow::Result;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).map_err(Into::into)
}