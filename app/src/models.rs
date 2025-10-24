use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub account_number: String,
    pub account_name: String,
    pub balance: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::accounts)]
pub struct NewAccount {
    pub account_number: String,
    pub account_name: String,
    pub balance: i64,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: i32,
    pub from_account_id: Option<i32>,
    pub to_account_id: Option<i32>,
    pub amount: i64,
    pub transaction_type: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::transactions)]
pub struct NewTransaction {
    pub from_account_id: Option<i32>,
    pub to_account_id: Option<i32>,
    pub amount: i64,
    pub transaction_type: String,
    pub description: Option<String>,
}
