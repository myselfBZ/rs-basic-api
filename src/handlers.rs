use std::sync::{Arc};
use serde::{Deserialize};
use tokio::sync::Mutex;
use crate::state::MyState;
use crate::response::*;
use axum::{Json, extract::{Path, State}, http::{self, StatusCode}};

pub async fn delete_user(
    State(mu): State<Arc<Mutex<MyState>>>, 
    Path(id): Path<String>) -> ApiResponse {
    let mut state = mu.lock().await;
    if !state.users.contains_key(&id){
        return ApiResponse::Error(http::StatusCode::NOT_FOUND, ErrorMessage { message: "user not found" })
    }
    state.users.remove(&id);
    return ApiResponse::NoContent
}

pub async fn get_user(State(mu): State<Arc<Mutex<MyState>>>, Path(id): Path<String>) -> ApiResponse {
    let state = mu.lock().await;
    if let Some(u) = state.users.get(&id) {
        return ApiResponse::JsonData(
            http::StatusCode::OK,
            u.clone()
        )
    } else {
        return ApiResponse::Error(
            http::StatusCode::NOT_FOUND, 
            ErrorMessage { message: "user not found" }
        )
    }
}

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    name: String
}

pub async fn update_user(
    State(mu): State<Arc<Mutex<MyState>>>, 
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserPayload>
) -> ApiResponse {
    let mut state = mu.lock().await;
    if !state.users.contains_key(&id){
        return ApiResponse::Error(http::StatusCode::NOT_FOUND, ErrorMessage { message: "user not found" })
    }
    state.users.insert(id.clone(), User { id: id.clone(), name: payload.name.clone() });
    return ApiResponse::JsonData(http::StatusCode::OK, User { 
        id: id.clone(), 
        name: payload.name.clone() 
    })
}

#[derive(Deserialize)]
pub struct CreateUser {
    name: String
}

pub async fn create_user(
    State(mu): State<Arc<Mutex<MyState>>>, 
    Json(payload): Json<CreateUser>
    ) -> ApiResponse {
    let id = uuid::Uuid::new_v4();
    let new_user = User{
        id: id.to_string(),
        name: payload.name
    };
    let mut state = mu.lock().await;
    state.users.insert(id.to_string(), new_user.clone());
    return ApiResponse::JsonData(
        http::StatusCode::CREATED,
        new_user
    )
}

pub async fn faulty_response() -> ApiResponse {
    ApiResponse::Error(StatusCode::NOT_FOUND, ErrorMessage{
        message: "user not found"
    })
}
