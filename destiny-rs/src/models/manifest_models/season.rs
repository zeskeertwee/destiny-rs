use {
    crate::models::{
        types::*,
        manifest_models::DisplayProperties,
    },
    serde::Deserialize
};

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
    pub season_pass_hash: Option<Hash>,
    /// mapped to [`Progression`](crate::models::manifest::ManifestKey::Progression)
    pub season_pass_progression_hash: Option<Hash>,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub artifact_item_hash: Option<Hash>,
    /// mapped to [`PresentationNode`](crate::models::manifest::ManifestKey::PresentationNode)
    pub seal_presentation_node_hash: Option<Hash>,
    /// mapped to [`PresentationNode`](crate::models::manifest::ManifestKey::PresentationNode)
    pub seasonal_challenges_presentation_node_hash: Option<Hash>,
    pub preview: Option<SeasonPreview>,
    pub hash: Hash,
    pub index: Int32,
    pub redacted: bool,
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