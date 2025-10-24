use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub account_number: String,
    pub account_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::accounts)]
pub struct NewAccount {
    pub account_number: String,
    pub account_name: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::ledger_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LedgerEvent {
    pub id: i32,
    pub account_id: i32,
    pub event_type: String,
    pub amount: i64,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::ledger_events)]
pub struct NewLedgerEvent {
    pub account_id: i32,
    pub event_type: String,
    pub amount: i64,
    pub description: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::account_balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountBalance {
    pub id: i32,
    pub account_id: i32,
    pub balance: i64,
    pub snapshot_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::account_balances)]
pub struct NewAccountBalance {
    pub account_id: i32,
    pub balance: i64,
}
