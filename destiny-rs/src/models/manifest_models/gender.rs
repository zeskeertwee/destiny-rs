use {
    crate::models::{
        types::*,
        manifest_models::DestinyDisplayProperties,
    },
    serde::Deserialize
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyGenderDefinition.html#schema_Destiny-Definitions-DestinyGenderDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gender {
    pub gender_type: Int32,
    pub display_properties: DestinyDisplayProperties,
    pub hash: Hash,
    pub index: Int32,
    pub redacted: bool,
}