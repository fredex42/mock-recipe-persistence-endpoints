use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CollectionKind {
    #[serde(rename="saved")]
    Saved,
    #[serde(rename="cooked")]
    Cooked,
    #[serde(rename="recentlyViewed")]
    RecentlyViewed,
    #[serde(rename="userCreated")]
    UserCreated
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionResponse {
    pub id: String,
    #[serde(rename="collectionType")]
    pub collection_type:CollectionKind,
    #[serde(rename="lastModified")]
    pub last_modified: time::OffsetDateTime   //also in header
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionsResponse {
    pub collections:Vec<CollectionResponse>
}