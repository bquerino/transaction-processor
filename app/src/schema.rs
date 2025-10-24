// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[max_length = 255]
        account_number -> Varchar,
        #[max_length = 255]
        account_name -> Varchar,
        balance -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        from_account_id -> Nullable<Int4>,
        to_account_id -> Nullable<Int4>,
        amount -> Int8,
        #[max_length = 50]
        transaction_type -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(accounts, transactions,);
