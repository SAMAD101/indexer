use diesel::prelude::*;
use crate::schema::transactions;

#[derive(Queryable, Insertable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub signature: String,
    pub slot: i64,
    pub err: Option<String>,
    pub memo: Option<String>,
    pub block_time: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
}