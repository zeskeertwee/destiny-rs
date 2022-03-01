use {
    crate::models::{
        types::*,
        manifest_models::DestinyDisplayProperties,
    },
    serde::Deserialize
};
use crate::models::manifest::ManifestKey;
use crate::traits::manifest_key::ManifestTableKey;

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyGenderDefinition.html#schema_Destiny-Definitions-DestinyGenderDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gender {
    pub gender_type: Int32,
    pub display_properties: DestinyDisplayProperties,
    pub hash: Hash<Self>,
    pub index: Int32,
    pub redacted: bool,
}

impl ManifestTableKey for Gender {
    const TABLE_KEY: ManifestKey = ManifestKey::Gender;
}