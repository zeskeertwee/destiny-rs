use {
    crate::models::{
        types::*,
        manifest_models::DestinyDisplayProperties,
    },
    serde::Deserialize,
};
use crate::models::manifest::ManifestKey;
use crate::traits::manifest_key::ManifestTableKey;

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyRaceDefinition.html#schema_Destiny-Definitions-DestinyGenderDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    // TODO: hashmaps
    pub display_properties: DestinyDisplayProperties,
    pub race_type: Int32,
    pub hash: Hash<Self>,
    pub index: Int32,
    pub redacted: bool,
}

impl ManifestTableKey for Race {
    const TABLE_KEY: ManifestKey = ManifestKey::Race;
}