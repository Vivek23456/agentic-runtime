use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    routes::{
        health::health,
        tasks::create_task,
    },
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tasks", post(create_task))
        .with_state(state)
}