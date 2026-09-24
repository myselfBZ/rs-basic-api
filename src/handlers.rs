use std::sync::{Arc};
use serde::{Deserialize};
use crate::{db::users::CreateUserParam, state::MyState};
use crate::db::users;
use crate::response::*;
use crate::entities::User;
use axum::{Json, extract::{Path, State}, http::{StatusCode}};

pub async fn delete_user(
    State(state): State<Arc<MyState>>, 
    Path(id): Path<uuid::Uuid>) -> ApiResponse {
    if let Some(e) = users::del(&state.db, id).await {
        ApiResponse::DynError(StatusCode::INTERNAL_SERVER_ERROR, ErrorMessageV2 { message: e.to_string() })
    } else {
        ApiResponse::NoContent
    }     
}

pub async fn get_user(State(state): State<Arc<MyState>>, Path(id): Path<uuid::Uuid>) -> ApiResponse {
    match users::get_user(&state.db, id).await {
        Ok(u) => ApiResponse::JsonData(StatusCode::OK, u.clone()),
        Err(_) => ApiResponse::Error(StatusCode::NOT_FOUND, ErrorMessage { message: "user not found" })
    }
}

// #[derive(Deserialize)]
// pub struct UpdateUserPayload {
//     name: String
// }

// pub async fn update_user(
//     State(mu): State<Arc<Mutex<MyState>>>, 
//     Path(id): Path<String>,
//     Json(payload): Json<UpdateUserPayload>
// ) -> ApiResponse {
//     let mut state = mu.lock().await;
//     if !state.users.contains_key(&id){
//         return ApiResponse::Error(http::StatusCode::NOT_FOUND, ErrorMessage { message: "user not found" })
//     }
//     state.users.insert(id.clone(), User { id: id.clone(), name: payload.name.clone() });
//     return ApiResponse::JsonData(http::StatusCode::OK, User { 
//         id: id.clone(), 
//         name: payload.name.clone() 
//     })
// }

#[derive(Deserialize)]
pub struct CreateUser {
    name: String
}

pub async fn create_user(
    State(state): State<Arc<MyState>>, 
    Json(payload): Json<CreateUser>
    ) -> ApiResponse {
    let param = CreateUserParam{
        name: payload.name.clone()
    };
    match users::create_user(&state.db, param).await {
        Ok(id) => ApiResponse::JsonData(StatusCode::CREATED, User { id: id, name: payload.name }),
        Err(_) => ApiResponse::Error(StatusCode::INTERNAL_SERVER_ERROR, ErrorMessage { message: "couldn't create user" })
    }
}

pub async fn faulty_response() -> ApiResponse {
    ApiResponse::Error(StatusCode::NOT_FOUND, ErrorMessage{
        message: "user not found"
    })
}
