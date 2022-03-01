use {
    crate::models::types::*,
    serde::Deserialize
};
use crate::models::manifest::ManifestKey;
use crate::traits::manifest_key::ManifestTableKey;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Progression {
    /// mapped to [`Progression`](crate::models::manifest::ManifestKey::Progression)
    pub progression_hash: Hash<Self>,
    pub daily_progress: Int32,
    pub daily_limit: Int32,
    pub weekly_progress: Int32,
    pub weekly_limit: Int32,
    pub current_progress: Int32,
    pub level: Int32,
    pub level_cap: Int32,
    pub step_index: Int32,
    pub progress_to_next_level: Int32,
    pub next_level_at: Int32,
    pub current_reset_count: Option<Int32>,
    pub season_resets: Option<Vec<ProgressionResetEntry>>,
    pub reward_item_states: Option<Vec<Int32>>,
}

impl ManifestTableKey for Progression {
    const TABLE_KEY: ManifestKey = ManifestKey::Progression;
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProgressionResetEntry {
    pub season: Int32,
    pub resets: Int32,
}