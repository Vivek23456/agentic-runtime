use std::{
    net::SocketAddr,
    
};



use crate::{
         
    bootstrap::bootstrap,

    router::create_router,
    
};

mod bootstrap;
mod models;
mod router;
mod routes;
mod state;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let state = bootstrap().await;

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