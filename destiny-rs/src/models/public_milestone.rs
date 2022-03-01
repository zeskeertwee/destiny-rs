use {
    crate::models::{
        types::*,
        activity_mode::ActivityMode,
    },
    serde::Deserialize,
    std::collections::HashMap,
};
use crate::models::manifest_models::InventoryItem;

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestone.html#schema_Destiny-Milestones-DestinyPublicMilestone)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestone {
    /// mapped to [`Milestone`](crate::models::manifest::ManifestKey::Milestone)
    // TODO
    pub milestone_hash: Uint32,
    #[serde(default = "serde_empty_vec")]
    pub availible_quests: Vec<PublicMilestoneQuest>,
    #[serde(default = "serde_empty_vec")]
    pub activities: Vec<PublicMilestoneChallengeActivity>,
    #[serde(default = "serde_empty_vec")]
    #[deprecated(note = "Use the `vendors` field instead")]
    pub vendor_hashes: Vec<Uint32>,
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
    // TODO
    pub quest_item_hash: Uint32,
    pub activity: PublicMilestoneActivity,
    pub challenges: Vec<PublicMilestoneChallenge>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneActivity.html#schema_Destiny-Milestones-DestinyPublicMilestoneActivity)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneActivity {
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    // TODO
    pub activity_hash: Uint32,
    #[serde(default = "serde_empty_vec")]
    /// mapped to [`ActivityModifier`](crate::models::manifest::ManifestKey::ActivityModifier)
    // TODO
    pub modifier_hashes: Vec<Uint32>,
    pub variants: Vec<PublicMilestoneActivityVariant>,
    /// mapped to [`ActivityMode`](crate::models::manifest::ManifestKey::ActivityMode)
    // TODO
    pub activity_mode_hash: Option<Uint32>,
    pub activity_mode_type: Option<ActivityMode>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneActivityVariant.html#schema_Destiny-Milestones-DestinyPublicMilestoneActivityVariant)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneActivityVariant {
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    // TODO
    pub activity_hash: Uint32,
    /// mapped to [`ActivityMode`](crate::models::manifest::ManifestKey::ActivityMode)
    // TODO
    pub activity_mode_hash: Option<Uint32>,
    pub activity_mode_type: Option<ActivityMode>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneChallenge.html#schema_Destiny-Milestones-DestinyPublicMilestoneChallenge)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneChallenge {
    /// mapped to [`Objective`](crate::models::manifest::ManifestKey::Objective)
    // TODO
    pub objective_hash: Uint32,
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    // TODO
    pub activity_hash: Option<Uint32>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneChallengeActivity.html#schema_Destiny-Milestones-DestinyPublicMilestoneChallengeActivity)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneChallengeActivity {
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    // TODO
    pub activity_hash: Uint32,
    pub challenge_objective_hashes: Vec<Uint32>,
    #[serde(default = "serde_empty_vec")]
    /// mapped to [`ActivityModifier`](crate::models::manifest::ManifestKey::ActivityModifier)
    // TODO
    pub modifier_hashes: Vec<Uint32>,
    pub loadout_requirement_index: Option<Int32>,
    #[serde(default = "serde_empty_vec")]
    pub phase_hashes: Vec<Uint32>,
    #[serde(default = "serde_empty_map")]
    pub boolean_activity_options: HashMap<String, bool>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Milestones-DestinyPublicMilestoneVendor.html#schema_Destiny-Milestones-DestinyPublicMilestoneVendor)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMilestoneVendor {
    /// mapped to [`Vendor`](crate::models::manifest::ManifestKey::Vendor)
    // TODO
    pub vendor_hash: Uint32,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub preview_item_hash: Option<Hash<InventoryItem>>
}