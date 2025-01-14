use models::{CollectionResponse, CollectionsResponse};
use time::OffsetDateTime;

mod models;

pub fn gen_user_collections(timestamp: OffsetDateTime) -> CollectionsResponse {
    CollectionsResponse{
        collections: vec![
            CollectionResponse{
                id: "F8895D13-CCB2-4864-9DE6-C35A1FC943BE".into(),
                collection_type: models::CollectionKind::Saved,
                last_modified: timestamp,
            },
            CollectionResponse{
                id: "22468120-81C4-4E4A-8B9D-71AEE5E25C40".into(),
                collection_type: models::CollectionKind::Cooked,
                last_modified: timestamp,
            },
        ]
    }
}