use axum::{http::StatusCode, response::IntoResponse, Json};
mod responses;
use responses::GenericResponse;

pub async fn generic404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(GenericResponse{
            status: "not_found".into(),
            detail: Some("Bad URL".into())
        })
    )
}