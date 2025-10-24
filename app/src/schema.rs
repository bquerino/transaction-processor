// @generated automatically by Diesel CLI.

diesel::table! {
    account_balances (id) {
        id -> Int4,
        account_id -> Int4,
        balance -> Int8,
        snapshot_at -> Timestamp,
    }
}

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[max_length = 255]
        account_number -> Varchar,
        #[max_length = 255]
        account_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    ledger_events (id) {
        id -> Int4,
        account_id -> Int4,
        #[max_length = 20]
        event_type -> Varchar,
        amount -> Int8,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(account_balances -> accounts (account_id));
diesel::joinable!(ledger_events -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(account_balances, accounts, ledger_events,);
