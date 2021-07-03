use {
    crate::models::{
        types::*,
        manifest_models::DestinyDisplayProperties,
    },
    serde::Deserialize
};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationNode {
    pub display_properties: DestinyDisplayProperties,
    pub original_icon: String,
    pub root_view_icon: String,
    pub node_type: Int32,
    pub scope: Int32,
    /// mapped to [`Objective`](crate::models::manifest::ManifestKey::Objective)
    pub objective_hash: Option<Hash>,
    /// mapped to [`Record`](crate::models::manifest::ManifestKey::Record)
    pub completion_record_hash: Option<Hash>,
    pub children: PresentationNodeChildrenBlock,
    pub display_style: Int32,
    pub screen_style: Int32,
    pub requirements: PresentationNodeRequirements,
    pub disable_child_subscreen_navigation: bool,
    pub max_category_record_score: Int32,
    pub presentation_node_type: Int32,
    pub trait_ids: Vec<String>,
    /// mapped to [`Trait`](crate::models::manifest::ManifestKey::Trait)
    pub trait_hashes: Vec<Hash>,
    /// mapped to [`PresentationNode`](crate::models::manifest::ManifestKey::PresentationNode)
    pub parent_node_hashes: Vec<Hash>,
    pub hash: Hash,
    pub index: Int32,
    pub redacted: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Presentation-DestinyPresentationNodeRequirementsBlock.html#schema_Destiny-Definitions-Presentation-DestinyPresentationNodeRequirementsBlock)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationNodeRequirements {
    pub entitlement_unavailable_message: String,
}

// TODO: child struct mapped to docs
/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Presentation-DestinyPresentationNodeChildrenBlock.html#schema_Destiny-Definitions-Presentation-DestinyPresentationNodeChildrenBlock)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationNodeChildrenBlock {
    pub presentation_nodes: Vec<PresentationNodeChildEntry>,
    pub collectibles: Vec<PresentationNodeCollectibleChildEntry>,
    pub records: Vec<PresentationNodeRecordChildEntry>,
    pub metrics: Vec<PresentationNodeMetricChildEntry>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Presentation-DestinyPresentationNodeChildEntry.html#schema_Destiny-Definitions-Presentation-DestinyPresentationNodeChildEntry)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationNodeChildEntry {
    pub presentation_node_hash: Hash,
}
/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Presentation-DestinyPresentationNodeCollectibleChildEntry.html#schema_Destiny-Definitions-Presentation-DestinyPresentationNodeCollectibleChildEntry)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationNodeCollectibleChildEntry {
    pub collectible_hash: Hash
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Presentation-DestinyPresentationNodeRecordChildEntry.html#schema_Destiny-Definitions-Presentation-DestinyPresentationNodeRecordChildEntry)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationNodeRecordChildEntry {
    pub record_hash: Hash,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Presentation-DestinyPresentationNodeMetricChildEntry.html#schema_Destiny-Definitions-Presentation-DestinyPresentationNodeMetricChildEntry)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationNodeMetricChildEntry {
    pub metric_hash: Hash,
}