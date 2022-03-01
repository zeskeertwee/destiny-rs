use {
    crate::models::{
        types::*,
        manifest_models::DestinyDisplayProperties,
    },
    serde::Deserialize,
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyRaceDefinition.html#schema_Destiny-Definitions-DestinyGenderDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    // TODO: hashmaps
    pub display_properties: DestinyDisplayProperties,
    pub race_type: Int32,
    pub hash: Hash,
    pub index: Int32,
    pub redacted: bool,
}