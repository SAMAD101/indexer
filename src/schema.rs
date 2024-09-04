// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (pubkey) {
        pubkey -> Text,
        lamports -> Int8,
        owner -> Text,
        executable -> Bool,
        rent_epoch -> Int8,
        data -> Bytea,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    transactions (signature) {
        signature -> Text,
        slot -> Int8,
        err -> Nullable<Text>,
        memo -> Nullable<Text>,
        block_time -> Nullable<Int8>,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
