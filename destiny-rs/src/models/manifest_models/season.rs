use {
    crate::models::{
        types::*,
        manifest_models::DisplayProperties,
    },
    serde::Deserialize
};
use crate::models::manifest::ManifestKey;
use crate::models::manifest_models::{InventoryItem, PresentationNode};
use crate::traits::manifest_key::ManifestTableKey;

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Seasons-DestinySeasonDefinition.html#schema_Destiny-Definitions-Seasons-DestinySeasonDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub display_properties: DisplayProperties,
    pub background_image_path: Option<String>,
    pub season_number: Int32,
    #[serde(deserialize_with = "uint32_from_str")]
    pub start_time_in_seconds: Uint32,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub start_date: Option<APIdateTime>,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub end_date: Option<APIdateTime>,
    /// mapped to [`SeasonPass`](crate::models::manifest::ManifestKey::SeasonPass)
    // TODO
    pub season_pass_hash: Option<Uint32>,
    /// mapped to [`Progression`](crate::models::manifest::ManifestKey::Progression)
    // TODO
    pub season_pass_progression_hash: Option<Uint32>,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub artifact_item_hash: Option<Hash<InventoryItem>>,
    /// mapped to [`PresentationNode`](crate::models::manifest::ManifestKey::PresentationNode)
    pub seal_presentation_node_hash: Option<Hash<PresentationNode>>,
    /// mapped to [`PresentationNode`](crate::models::manifest::ManifestKey::PresentationNode)
    pub seasonal_challenges_presentation_node_hash: Option<Hash<PresentationNode>>,
    pub preview: Option<SeasonPreview>,
    pub hash: Hash<Self>,
    pub index: Int32,
    pub redacted: bool,
}

impl ManifestTableKey for Season {
    const TABLE_KEY: ManifestKey = ManifestKey::Season;
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Seasons-DestinySeasonPreviewDefinition.html#schema_Destiny-Definitions-Seasons-DestinySeasonPreviewDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonPreview {
    pub description: String,
    pub link_path: String,
    pub video_link: Option<String>,
    pub images: Vec<SeasonPreviewImage>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Seasons-DestinySeasonPreviewImageDefinition.html#schema_Destiny-Definitions-Seasons-DestinySeasonPreviewImageDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonPreviewImage {
    pub thumbnail_image: String,
    pub high_res_image: String,
}