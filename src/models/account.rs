use diesel::prelude::*;
use crate::schema::accounts;

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub pubkey: String,
    pub lamports: i64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: i64,
    pub data: Vec<u8>,
    pub updated_at: chrono::NaiveDateTime,
}