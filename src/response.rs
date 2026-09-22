use axum::response::{IntoResponse};
use axum::http::StatusCode;
use axum::Json;
#[derive(Clone, serde::Serialize)]
pub struct User {
    pub id: String,
    pub name: String
}


#[derive(serde::Serialize)]
pub struct ErrorMessage {
    pub message: &'static str
}

pub enum ApiResponse {
    Error(StatusCode, ErrorMessage),
    JsonData(StatusCode, User),
    NoContent
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::JsonData(c, d) => (c, Json(d)).into_response(),
            Self::Error(c, m) => (c, Json(m)).into_response(),
            Self::NoContent => (StatusCode::NO_CONTENT).into_response()
        }
    }
}

