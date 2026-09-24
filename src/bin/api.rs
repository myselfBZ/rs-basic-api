use std::sync::Arc;
use axum::{Router, routing::{delete, get, post}};
use basic_api::state::MyState;
use basic_api::handlers;
use basic_api::middleware;

#[tokio::main]
async fn main() {
    let app_state = MyState::new("postgresql://postgres:pass@localhost:5432/basic_api?sslmode=disable").await;
    let shared = Arc::new(app_state);
    let app = Router::new()
        .route("/users/{id}", get(handlers::get_user))
        .route("/users", post(handlers::create_user))
        // .route("/users/{id}", put(handlers::update_user))
        .route("/users/{id}", delete(handlers::delete_user))
        // .route("/error", get(handlers::faulty_response))
        .layer(axum::middleware::from_fn(middleware::logger_middleware))
        .with_state(shared);
 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.expect("server failed");
}
