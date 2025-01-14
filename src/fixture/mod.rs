use std::collections::HashMap;

use models::{CollectionResponse, CollectionsResponse};
use time::OffsetDateTime;

mod models;

#[derive(clap::ValueEnum, Debug, Clone, Default)]
pub enum Environment {
    CODE,
    #[default]
    PROD
}

impl ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            Environment::CODE=>"code".into(),
            Environment::PROD=>"prod".into()
        }
    }
}

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

pub const CodeRecipesSavedSample:[&str; 6] = [
    "99ea87d53eb3dc2f2f445b38919d9b9cbda4b7b1",
    "f7c7d8f4d84f8792fb3db7e8a8d01dd0d0f5f44e",
    "0a4058e84d82baafa0b5e3f9592ccbb338948436",
    "87ff572716ff27ce79af69103853037129d96278",
    "53011c2b714858980de757ea640f63cd0be3fa4d",
    "4e69b3b3563d4c7b8171875029fb55ea"
];

pub const CodeRecipesCookedSample:[&str; 5] = [
    "784df601c67b345c66b1421dd4bb509de6e56506",
    "70265dd004fc4b03a5cb50f972fe13f5",
    "52162dbaf825914f151a60b4039dce6098831ba0",
    "0bba989e6300c009d6deec7c305f321cf1592446",
    "47153f13da3db88470e1d13da68fbcdc1971674f"
];

pub const ProdRecipesSavedSample:[&str; 6] = [
    "ed9e148c614d47f0b236f5ce7113d196",
    "cedbcd3f945edab2082929cdb32b53be0f60ce65",
    "abd71bb34eda489ca830d3d7688dc5f0",
    "0b1f06a6a3c54e1c8deb23bdc7d1d9ae",
    "c3d429e206c24e4f8bb362913e959c77",
    "d5a731c27edd44ff9df5b9dd4f8d983e"
];

const ProdRecipesCookedSample:[&str; 5] = [
    "01d206bc58b54044836e3c5046101062",
    "cfac27aee0d01ea8718a61fd1d23f441402dee17",
    "17ad58e0cffc482b8b0e591a94e30809",
    "b01c05a481ba00f9ae21a4f2bf11e137b6609c42",
    "9a6b1e956f774667ad7562d6410ab73e"
];

pub struct MutableStaticData {
    env: Environment,
    collections:HashMap<String, Vec<String>>,
}

impl MutableStaticData {
    pub fn new(env:Environment) -> MutableStaticData {
        match env {
            Environment::CODE=>{
                let mut collections:HashMap<String, Vec<String>> = HashMap::new();
                let saved:Vec<String> = CodeRecipesSavedSample.into_iter().map(|v| v.to_string()).collect();
                let cooked:Vec<String> = CodeRecipesCookedSample.into_iter().map(|v| v.to_string()).collect();

                collections.insert("F8895D13-CCB2-4864-9DE6-C35A1FC943BE".into(), saved);
                collections.insert("22468120-81C4-4E4A-8B9D-71AEE5E25C40".into(), cooked);

                MutableStaticData{
                    env: env,
                    collections: collections,
                }
            }
            Environment::PROD=>{
                let mut collections:HashMap<String, Vec<String>> = HashMap::new();
                let saved:Vec<String> = ProdRecipesSavedSample.into_iter().map(|v| v.to_string()).collect();
                let cooked:Vec<String> = ProdRecipesCookedSample.into_iter().map(|v| v.to_string()).collect();
                collections.insert("F8895D13-CCB2-4864-9DE6-C35A1FC943BE".into(), saved);
                collections.insert("22468120-81C4-4E4A-8B9D-71AEE5E25C40".into(), cooked);

                MutableStaticData{
                    env: env,
                    collections: collections,
                }
            }
        }
    }
}