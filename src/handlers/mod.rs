use axum::{http::StatusCode, response::IntoResponse, Json};
mod responses;
use responses::GenericResponse;
use crate::fixture::*;

pub async fn generic404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(GenericResponse{
            status: "not_found".into(),
            detail: Some("Bad URL".into())
        })
    )
}

pub async fn get_user_collections() -> impl IntoResponse {
    let now = time::OffsetDateTime::now_utc();

    let collections = gen_user_collections(now);

    (
        StatusCode::OK,
        Json(collections)
    )
}