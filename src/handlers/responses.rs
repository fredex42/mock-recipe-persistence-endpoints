use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub detail: Option<String>
}


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
