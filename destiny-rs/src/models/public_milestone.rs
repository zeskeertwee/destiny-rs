use {
    crate::models::{
        types::*,
        activity_mode::ActivityMode,
    },
    serde::Deserialize,
    std::collections::HashMap,
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestone.html#schema_Destiny-Milestones-DestinyPublicMilestone)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestone {
    /// mapped to [`Milestone`](crate::models::manifest::ManifestKey::Milestone)
    pub milestone_hash: Hash,
    #[serde(default = "serde_empty_vec")]
    pub availible_quests: Vec<PublicMilestoneQuest>,
    #[serde(default = "serde_empty_vec")]
    pub activities: Vec<PublicMilestoneChallengeActivity>,
    #[serde(default = "serde_empty_vec")]
    pub vendor_hashes: Vec<Hash>,
    #[serde(default = "serde_empty_vec")]
    pub vendors: Vec<PublicMilestoneVendor>,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub start_date: Option<APIdateTime>,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub end_date: Option<APIdateTime>,
    pub order: Int32,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneQuest.html#schema_Destiny-Milestones-DestinyPublicMilestoneQuest)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneQuest {
    /// mapped to [`Milestone`](crate::models::manifest::ManifestKey::Milestone)
    pub quest_item_hash: Hash,
    pub activity: PublicMilestoneActivity,
    pub challenges: Vec<PublicMilestoneChallenge>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneActivity.html#schema_Destiny-Milestones-DestinyPublicMilestoneActivity)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneActivity {
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    pub activity_hash: Hash,
    #[serde(default = "serde_empty_vec")]
    /// mapped to [`ActivityModifier`](crate::models::manifest::ManifestKey::ActivityModifier)
    pub modifier_hashes: Vec<Hash>,
    pub variants: Vec<PublicMilestoneActivityVariant>,
    /// mapped to [`ActivityMode`](crate::models::manifest::ManifestKey::ActivityMode)
    pub activity_mode_hash: Option<Hash>,
    pub activity_mode_type: Option<ActivityMode>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneActivityVariant.html#schema_Destiny-Milestones-DestinyPublicMilestoneActivityVariant)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneActivityVariant {
    pub activity_hash: Hash,
    /// mapped to [`ActivityMode`](crate::models::manifest::ManifestKey::ActivityMode)
    pub activity_mode_hash: Option<Hash>,
    pub activity_mode_type: Option<ActivityMode>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneChallenge.html#schema_Destiny-Milestones-DestinyPublicMilestoneChallenge)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneChallenge {
    /// mapped to [`Objective`](crate::models::manifest::ManifestKey::Objective)
    pub objective_hash: Hash,
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    pub activity_hash: Option<Hash>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneChallengeActivity.html#schema_Destiny-Milestones-DestinyPublicMilestoneChallengeActivity)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneChallengeActivity {
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    pub activity_hash: Hash,
    pub challenge_objective_hashes: Vec<Hash>,
    #[serde(default = "serde_empty_vec")]
    /// mapped to [`ActivityModifier`](crate::models::manifest::ManifestKey::ActivityModifier)
    pub modifier_hashes: Vec<Hash>,
    pub loadout_requirement_index: Option<Int32>,
    #[serde(default = "serde_empty_vec")]
    pub phase_hashes: Vec<Hash>,
    #[serde(default = "serde_empty_map")]
    pub boolean_activity_options: HashMap<String, bool>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneVendor.html#schema_Destiny-Milestones-DestinyPublicMilestoneVendor)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneVendor {
    /// mapped to [`Vendor`](crate::models::manifest::ManifestKey::Vendor)
    pub vendor_hash: Hash,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub preview_item_hash: Option<Hash>
}