use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    routes::{
        health::health,
        tasks::{
            create_task,
            get_task,
            list_tasks,
        },
    },
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route(
    "/tasks",
    post(create_task).get(list_tasks),
)
.route(
    "/tasks/:id",
    get(get_task),
)
        .with_state(state)
}