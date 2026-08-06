use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    routes::{
        agents::{create_agent, get_agent, list_agents},
        health::health,
        executions::{
    create_execution,
    get_execution,
    list_executions,
},
        versions::{
            create_version,
            get_version,
            list_versions,
        },
    },
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
         
        .route(
            "/agents",
            post(create_agent).get(list_agents),
        )
        .route(
    "/versions",
    post(create_version).get(list_versions),
)
.route(
    "/versions/:id",
    get(get_version),
)
.route(
    "/executions",
    post(create_execution).get(list_executions),
)
.route(
    "/executions/:id",
    get(get_execution),
)
        .route(
            "/agents/:id",
            get(get_agent),
        )
        .with_state(state)
}