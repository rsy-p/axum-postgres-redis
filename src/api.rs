mod user;

use crate::AppState;
use axum::Router;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    let base_router = router_collect(state.clone());
    Router::new().nest(&state.config.context_path, base_router)
}

fn router_collect(state: Arc<AppState>) -> Router {
    Router::new().merge(user::router(state.clone()))
}
