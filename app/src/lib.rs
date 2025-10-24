pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn create_account(
    pool: &DbPool,
    account_number: String,
    account_name: String,
    initial_balance: i64,
) -> Result<models::Account, diesel::result::Error> {
    use crate::schema::accounts;

    let new_account = models::NewAccount {
        account_number,
        account_name,
        balance: initial_balance,
    };

    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .returning(models::Account::as_returning())
        .get_result(&mut conn)
}

pub fn create_transaction(
    pool: &DbPool,
    from_account_id: Option<i32>,
    to_account_id: Option<i32>,
    amount: i64,
    transaction_type: String,
    description: Option<String>,
) -> Result<models::Transaction, diesel::result::Error> {
    use crate::schema::transactions;

    let new_transaction = models::NewTransaction {
        from_account_id,
        to_account_id,
        amount,
        transaction_type,
        description,
    };

    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .returning(models::Transaction::as_returning())
        .get_result(&mut conn)
}

pub fn get_account_balance(
    pool: &DbPool,
    account_id: i32,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    accounts
        .find(account_id)
        .select(balance)
        .first(&mut conn)
}

pub fn list_accounts(
    pool: &DbPool,
) -> Result<Vec<models::Account>, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    accounts
        .select(models::Account::as_select())
        .load(&mut conn)
}

pub fn list_transactions(
    pool: &DbPool,
) -> Result<Vec<models::Transaction>, diesel::result::Error> {
    use crate::schema::transactions::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    transactions
        .select(models::Transaction::as_select())
        .load(&mut conn)
}
