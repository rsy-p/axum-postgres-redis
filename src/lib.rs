mod api;
pub mod generic;

pub use generic::*;

use std::sync::Arc;

pub async fn server(state: AppState) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", state.config.server_port);
    tracing::info!("Server listening on {}", addr);
    let router = api::router(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
