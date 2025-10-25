use crate::api::AppState;
use crate::application::commands::{
    CreateAccountCommand, CreateBalanceSnapshotCommand, CreateLedgerEventCommand,
};
use crate::application::queries::{GetAccountBalanceQuery, GetAccountQuery, ListAccountsQuery, ListLedgerEventsQuery};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

// DTOs for API requests/responses
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub account_number: String,
    pub account_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLedgerEventRequest {
    pub account_id: i32,
    pub event_type: String, // "DEBIT" or "CREDIT"
    pub amount: i64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListEventsQuery {
    pub account_id: Option<i32>,
}

// Error response helper
struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        error!("API error: {:?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": self.0.to_string()
            })),
        )
            .into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/// Create the API router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Account routes
        .route("/accounts", post(create_account))
        .route("/accounts", get(list_accounts))
        .route("/accounts/:id", get(get_account))
        .route("/accounts/:id/balance", get(get_account_balance))
        // Ledger event routes
        .route("/events", post(create_ledger_event))
        .route("/events", get(list_ledger_events))
        // Balance snapshot routes
        .route("/balances/snapshot", post(create_balance_snapshot))
        .with_state(state)
}

// Handlers

async fn create_account(
    State(state): State<AppState>,
    Json(req): Json<CreateAccountRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    info!("Creating account: {:?}", req);

    let command = CreateAccountCommand::new(req.account_number, req.account_name);

    let account = state.mediator.send_create_account(command).await?;

    Ok(Json(json!({
        "id": account.id,
        "account_number": account.account_number.value(),
        "account_name": account.account_name,
        "created_at": account.created_at,
        "updated_at": account.updated_at
    })))
}

async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, ApiError> {
    info!("Getting account: id={}", id);

    let query = GetAccountQuery::new(id);
    let account = state.mediator.send_get_account(query).await?;

    Ok(Json(json!({
        "id": account.id,
        "account_number": account.account_number.value(),
        "account_name": account.account_name,
        "created_at": account.created_at,
        "updated_at": account.updated_at
    })))
}

async fn list_accounts(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    info!("Listing all accounts");

    let query = ListAccountsQuery::new();
    let accounts = state.mediator.send_list_accounts(query).await?;

    let accounts_json: Vec<_> = accounts
        .into_iter()
        .map(|acc| {
            json!({
                "id": acc.id,
                "account_number": acc.account_number.value(),
                "account_name": acc.account_name,
                "created_at": acc.created_at,
                "updated_at": acc.updated_at
            })
        })
        .collect();

    Ok(Json(json!({
        "accounts": accounts_json,
        "count": accounts_json.len()
    })))
}

async fn get_account_balance(
    State(state): State<AppState>,
    Path(account_id): Path<i32>,
) -> Result<Json<serde_json::Value>, ApiError> {
    info!("Getting balance for account_id={}", account_id);

    let query = GetAccountBalanceQuery::new(account_id);
    let balance = state.mediator.send_get_account_balance(query).await?;

    Ok(Json(json!({
        "account_id": balance.account_id,
        "balance": balance.balance.value(),
        "snapshot_at": balance.snapshot_at
    })))
}

async fn create_ledger_event(
    State(state): State<AppState>,
    Json(req): Json<CreateLedgerEventRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    info!("Creating ledger event: {:?}", req);

    let command = CreateLedgerEventCommand::new(
        req.account_id,
        req.event_type,
        req.amount,
        req.description,
    );

    let event = state.mediator.send_create_ledger_event(command).await?;

    Ok(Json(json!({
        "id": event.id,
        "account_id": event.account_id,
        "event_type": event.event_type.to_string(),
        "amount": event.amount.value(),
        "description": event.description,
        "created_at": event.created_at
    })))
}

async fn list_ledger_events(
    State(state): State<AppState>,
    Query(params): Query<ListEventsQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    info!("Listing ledger events: {:?}", params);

    let query = if let Some(account_id) = params.account_id {
        ListLedgerEventsQuery::for_account(account_id)
    } else {
        ListLedgerEventsQuery::new()
    };

    let events = state.mediator.send_list_ledger_events(query).await?;

    let events_json: Vec<_> = events
        .into_iter()
        .map(|event| {
            json!({
                "id": event.id,
                "account_id": event.account_id,
                "event_type": event.event_type.to_string(),
                "amount": event.amount.value(),
                "description": event.description,
                "created_at": event.created_at
            })
        })
        .collect();

    Ok(Json(json!({
        "events": events_json,
        "count": events_json.len()
    })))
}

async fn create_balance_snapshot(
    State(state): State<AppState>,
    Json(account_id_json): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let account_id = account_id_json
        .get("account_id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("Missing account_id"))? as i32;

    info!("Creating balance snapshot for account_id={}", account_id);

    let command = CreateBalanceSnapshotCommand::new(account_id);
    let snapshot = state.mediator.send_create_balance_snapshot(command).await?;

    Ok(Json(json!({
        "id": snapshot.id,
        "account_id": snapshot.account_id,
        "balance": snapshot.balance.value(),
        "snapshot_at": snapshot.snapshot_at
    })))
}
