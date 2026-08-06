use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    routes::{
        agents::{create_agent,list_agents,get_agent},
        health::health,
       
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
            "/agents/:id",
            get(get_agent),
        )
        .with_state(state)
}