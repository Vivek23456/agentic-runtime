use std::{
    net::SocketAddr,
    sync::Arc,
};

use runtime::RuntimeManager;

use crate::{
    router::create_router,
    state::AppState,
};
use domain::AgentRegistry;

mod models;
mod router;
mod routes;
mod state;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let runtime = Arc::new(
        RuntimeManager::new(
            4,
            1024,
        ),
    );

    let state = AppState {
        runtime,
        agents: AgentRegistry::new(),
    };

    let app = create_router(state);

    let addr = SocketAddr::from(([127,0,0,1],8080));

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}