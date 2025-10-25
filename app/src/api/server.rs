use crate::api::{create_router, AppState};
use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

pub async fn start_server(state: AppState, port: u16) -> Result<()> {
    let app = create_router(state);

    let addr = format!("0.0.0.0:{}", port);
    info!("Starting REST API server on {}", addr);

    let listener = TcpListener::bind(&addr).await?;

    info!("✓ REST API server listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
