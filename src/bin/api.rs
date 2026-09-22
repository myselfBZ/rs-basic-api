use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use axum::{Router, routing::{delete, get, post, put}};
use basic_api::state::MyState;
use basic_api::handlers;
use basic_api::middleware;
use basic_api::response::User; 

#[tokio::main]
async fn main() {
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert(
         "ca1529dd-c347-4d71-9eb3-e4652def9053".to_string(), 
        User{id: "ca1529dd-c347-4d71-9eb3-e4652def9053".to_string(), name: "Alex".to_string()},
    );
    users.insert(
        "12606fb3-6595-4017-b913-76847486a6c7".to_string(),
        User{id: "12606fb3-6595-4017-b913-76847486a6c7".to_string(), name: "Johnatan".to_string()}
    );
    users.insert(
        "93fc00cf-b5c2-4fdf-8a66-5bdd73ef51db".to_string(),
        User{id: "93fc00cf-b5c2-4fdf-8a66-5bdd73ef51db".to_string(), name: "Sarah".to_string()}
    );
    let app_state = MyState{
        users: users,
    };
    let mu_shared = Mutex::new(app_state);
    let shared = Arc::new(mu_shared);
    let app = Router::new()
        .route("/users/{id}", get(handlers::get_user))
        .route("/users", post(handlers::create_user))
        .route("/users/{id}", put(handlers::update_user))
        .route("/users/{id}", delete(handlers::delete_user))
        .route("/error", get(handlers::faulty_response))
        .layer(axum::middleware::from_fn(middleware::logger_middleware))
        .with_state(shared);
 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.expect("server failed");
}
