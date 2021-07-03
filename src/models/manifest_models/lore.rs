use {
    crate::models::{
        types::*,
        manifest_models::DestinyDisplayProperties,
    },
    serde::Deserialize,
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Lore-DestinyLoreDefinition.html#schema_Destiny-Definitions-Lore-DestinyLoreDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lore {
    pub display_properties: DestinyDisplayProperties,
    pub subtitle: Option<String>,
    pub hash: Hash,
    pub index: Int32,
    pub redacted: bool,
}