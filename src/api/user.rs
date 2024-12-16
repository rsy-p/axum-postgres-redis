use crate::AppState;
use axum::extract::State;
use axum::{Json, Router};
use std::sync::Arc;
use axum::routing::get;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/public", get(hello_world)).with_state(state)
}

pub async fn hello_world(State(state): State<Arc<AppState>>) -> Json<String> {
    Json(state.config.context_path.to_owned())
}
