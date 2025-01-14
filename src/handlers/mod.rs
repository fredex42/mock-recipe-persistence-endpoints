use std::{ops::Deref, sync::Arc};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
mod responses;
use responses::{CollectionContentResponse, GenericResponse};
use tokio::sync::RwLock;
use crate::fixture::*;

pub type SharedState = Arc<RwLock<MutableStaticData>>;

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

pub async fn get_collection_content(
    Path(collection_id): Path<String>,
    Extension(shared_state): Extension<SharedState>
) -> impl IntoResponse {
    let now = time::OffsetDateTime::now_utc();

    let state_ref = shared_state.clone();
    let guarded_data = state_ref.read().await;
    let maybe_collections =  guarded_data.deref().collections.get(collection_id.as_str());

    match maybe_collections {
        None=>(
            StatusCode::NOT_FOUND,
            Json(GenericResponse{
                status: "not_found".into(),
                detail: Some("That collection ID does not exist".into())
            })
        ).into_response(),
        Some(collections)=>(
            StatusCode::OK,
            Json(CollectionContentResponse{
                content_type: responses::ContentKind::Recipe,
                content: collections.to_owned(),
                last_modified: Some(now),
            })
        ).into_response()
    }
}
