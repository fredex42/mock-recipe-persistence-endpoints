use serde::{Deserialize, Serialize};

//Note - use rfc2822 for last-modified and if-modified-since
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
    #[serde(rename="lastModified",serialize_with="time::serde::rfc3339::serialize")]
    pub last_modified: time::OffsetDateTime   //also in header
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionsResponse {
    pub collections:Vec<CollectionResponse>
}