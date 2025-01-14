use serde::{Deserialize, Serialize};
// use chrono::{serde::ts_milliseconds, TimeZone};
// use chrono::TimeZone::UTC;

#[derive(Serialize, Deserialize, Debug)]
pub enum ContentKind {
    Recipe,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionContentResponse {
    pub content:Vec<String>,
    #[serde(rename="contentType")]
    pub content_type: ContentKind,
    #[serde(rename="lastModified")]
    pub last_modified:Option<time::OffsetDateTime>   //also in header
}

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