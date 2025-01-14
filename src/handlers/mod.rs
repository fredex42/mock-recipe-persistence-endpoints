use std::{collections::{HashMap, HashSet}, ops::{Deref, DerefMut}, sync::Arc};
use axum::http;
use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse, Extension, Json};
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

///TODO - add limit and offset params
/// TODO - add if-modified-since behaviour
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

async fn add_to_state(state:SharedState, collection_id:&str, recipe_id_list:Vec<&str>) -> Result<(), (http::status::StatusCode, String)>{
    if recipe_id_list.is_empty() {
        return Err( (StatusCode::BAD_REQUEST, "no recipes to add".into()))
    }

    let state_ref = state.clone();
    let mut guarded_data = state_ref.write().await;
    
    match guarded_data.deref_mut().collections.get_mut(collection_id) {
        None=>Err( (StatusCode::NOT_FOUND, "collection did not exist".into()) ),
        Some(mutable_collection)=>{
            recipe_id_list.iter().for_each(|recipe_id| {
                mutable_collection.push(recipe_id.to_string());
            });
            mutable_collection.dedup();
            Ok( () )
        }
    }
}

async fn remove_from_state(state:SharedState, collection_id:&str, recipe_id_list:Vec<&str>) -> Result<(), (http::status::StatusCode, String)> {
    //yeah this should be much more DRY. So shoot me.
    if recipe_id_list.is_empty() {
        return Err( (StatusCode::BAD_REQUEST, "no recipes to add".into()))
    }
    
    let state_ref = state.clone();
    let mut guarded_data = state_ref.write().await;
    
    let targets:HashSet<&str> = HashSet::from_iter(recipe_id_list);
    
    match guarded_data.deref_mut().collections.get_mut(collection_id) {
        None=>Err( (StatusCode::NOT_FOUND, "collection did not exist".into()) ),
        Some(mutable_collection)=>{
            mutable_collection.retain(|id| !targets.contains(id.as_str()));
            Ok( () )
        }
    }
}

pub async fn put_to_collection(
    Query(params): Query<HashMap<String, String>>,
    Path(collection_id):Path<String>,
    Extension(shared_state): Extension<SharedState>,
) -> impl IntoResponse {
    //this, too, should be DRYer :shrug:
    let maybe_id_list:Option<Vec<&str>> = params.get("ids").map(|s| s.split(",").collect());

    match maybe_id_list {
        None=>(
            StatusCode::BAD_REQUEST,
            Json(GenericResponse{
                status: "bad_request".into(),
                detail: Some("you must provide ?ids= to indicate the ids to put".into())
            })
        ).into_response(),
        Some(id_list)=>{
            match add_to_state(shared_state, &collection_id, id_list).await {
                Ok(_)=>(
                    StatusCode::NO_CONTENT,
                    Json(GenericResponse{
                        status: "updated".into(),
                        detail: None,
                    })
                ).into_response(),
                Err((code, e))=>(
                    code,
                    Json(GenericResponse{
                        status: "not_found".into(),
                        detail: Some(e),
                    })
                ).into_response()
            }
        }
    }
}

pub async fn delete_from_collection(
    Query(params): Query<HashMap<String, String>>,
    Path(collection_id):Path<String>,
    Extension(shared_state): Extension<SharedState>,
) -> impl IntoResponse {
    let maybe_id_list:Option<Vec<&str>> = params.get("ids").map(|s| s.split(",").collect());

    match maybe_id_list {
        None=>(
            StatusCode::BAD_REQUEST,
            Json(GenericResponse{
                status: "bad_request".into(),
                detail: Some("you must provide ?ids= to indicate the ids to remove".into())
            })
        ).into_response(),
        Some(id_list)=>{
            match remove_from_state(shared_state, &collection_id, id_list).await {
                Ok(_)=>(
                    StatusCode::NO_CONTENT,
                    Json(GenericResponse{
                        status: "updated".into(),
                        detail: None,
                    })
                ).into_response(),
                Err((code, e))=>(
                    code,
                    Json(GenericResponse{
                        status: "not_found".into(),
                        detail: Some(e),
                    })
                ).into_response()
            }
        }
    }
}

mod test {
    use super::*;

    #[tokio::test]
    async fn test_add_to_state() -> Result<(), String> {
        let mut fixture:HashMap<String, Vec<String>> = HashMap::new();
        fixture.insert("collection1".into(), vec!["recep1".into(),"recep2".into()]);
        fixture.insert("collection2".into(), vec!["recep3".into(), "recep4".into()]);

        let state = Arc::new(
            RwLock::new(
                MutableStaticData{
                    _env: Environment::CODE,
                    collections: fixture
                }
            )
        );

        let result = add_to_state(state.clone(), "collection2", vec!["recep5"]).await;
        let new_state = state.read().await;

        match result {
            Ok(_)=>{
                assert_eq!(new_state.collections.len(), 2);
                assert_eq!(new_state.collections.get("collection1").map(|c| c.len()), Some(2));
                assert_eq!(new_state.collections.get("collection2").map(|c| c.len()), Some(3));
                assert_eq!(new_state.collections.get("collection2").map(|c| c.contains(&"recep3".to_string())), Some(true));
                assert_eq!(new_state.collections.get("collection2").map(|c| c.contains(&"recep4".to_string())), Some(true));
                assert_eq!(new_state.collections.get("collection2").map(|c| c.contains(&"recep5".to_string())), Some(true));
                
                Ok( () )
            },
            Err(e)=>{
                Err(format!("Unexpected return value {:?}", e)).into()
            }
        }
    }

    #[tokio::test]
    async fn test_remove_from_state() -> Result<(), String> {
        let mut fixture:HashMap<String, Vec<String>> = HashMap::new();
        fixture.insert("collection1".into(), vec!["recep1".into(),"recep2".into()]);
        fixture.insert("collection2".into(), vec!["recep3".into(), "recep4".into(), "recep5".into()]);

        let state = Arc::new(
            RwLock::new(
                MutableStaticData{
                    _env: Environment::CODE,
                    collections: fixture
                }
            )
        );

        let result = remove_from_state(state.clone(), "collection2", vec!["recep3"]).await;
        let new_state = state.read().await;

        match result {
            Ok(_)=>{
                assert_eq!(new_state.collections.len(), 2);
                assert_eq!(new_state.collections.get("collection1").map(|c| c.len()), Some(2));
                assert_eq!(new_state.collections.get("collection2").map(|c| c.len()), Some(2));
                assert_eq!(new_state.collections.get("collection2").map(|c| c.contains(&"recep3".to_string())), Some(false));
                assert_eq!(new_state.collections.get("collection2").map(|c| c.contains(&"recep4".to_string())), Some(true));
                assert_eq!(new_state.collections.get("collection2").map(|c| c.contains(&"recep5".to_string())), Some(true));

                Ok( () )
            },
            Err(e)=>{
                Err(format!("Unexpected return value {:?}", e)).into()
            }
        }
    }
}