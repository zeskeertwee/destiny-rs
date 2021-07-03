use {
    crate::models::{
        types::*,
        manifest_models::DestinyDisplayProperties,
    },
    serde::Deserialize,
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Records-DestinyRecordDefinition.html#schema_Destiny-Definitions-Records-DestinyRecordDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub display_properties: DestinyDisplayProperties,
    pub scope: Int32,
    pub presentation_info: Option<PresentationChildBlock>,
    // TODO: mapped to doc
    pub lore_hash: Option<Hash>,
    pub objective_hashes: Vec<Hash>,
    pub record_value_style: Int32,
    pub for_title_gilding: bool,
    // TODO: titleInfo, completionInfo, stateInfo, requirements, expirationInfo, intervalInfo, rewardItems
    pub presentation_node_type: Int32,
    pub trait_ids: Vec<String>,
    pub trait_hashes: Vec<Hash>,
    pub parent_node_hashes: Vec<Hash>,
    pub hash: Hash,
    pub index: Int32,
    pub redacted: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Presentation-DestinyPresentationChildBlock.html#schema_Destiny-Definitions-Presentation-DestinyPresentationChildBlock)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationChildBlock {
    pub presentation_node_type: Int32,
    // TODO: mapped to doc
    pub parent_presentation_node_hashes: Vec<Hash>,
    pub display_style: Int32,
}