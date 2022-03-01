use serde::{Serialize, Deserialize};
use serde_json;
use tokio_test::block_on;
use crate::models::api::DestinyAPI;
use crate::models::locale::Locale;
use crate::models::manifest_models::Class;
use crate::models::types::Hash;

// this is the hash for the Hunter class in DestinyClassDefinition
const JSON_DATA: &'static str = r#"{"hash":671679327}"#;

#[derive(Deserialize, Serialize)]
struct Data {
    hash: Hash<Class>
}

#[test]
fn test_hash_serialization() {
    let api = DestinyAPI::new(&std::env::var("BUNGIE_API_KEY").unwrap()).unwrap();
    let mut manifest_path = crate::get_recommended_manifest_path().expect("Unable to get manifest path");
    let m = block_on(api.manifest(manifest_path, Locale::English)).unwrap();

    let data: Data = serde_json::from_str(JSON_DATA).unwrap();
    assert_eq!(data.hash.query(&m).unwrap().display_properties.name, "Hunter");
}