use {
    crate::models::{
        types::*,
        membership::MembershipType,
    },
    serde::Deserialize,
    std::collections::HashMap
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyHistoricalStatsByPeriod.html#schema_Destiny-HistoricalStats-DestinyHistoricalStatsByPeriod)
#[derive(Debug, Deserialize)]
pub struct HistoricalStatsByPeriod {
    #[serde(rename = "allTime")]
    pub all_time: HashMap<String, HistoricalStatsValue>,
    #[serde(rename = "allTimeTier1")]
    pub all_time_tier1: Option<HashMap<String, HistoricalStatsValue>>,
    #[serde(rename = "allTimeTier2")]
    pub all_time_tier2: Option<HashMap<String, HistoricalStatsValue>>,
    #[serde(rename = "allTimeTier3")]
    pub all_time_tier3: Option<HashMap<String, HistoricalStatsValue>>,
    pub daily: Option<HistoricalStatsPeriodGroup>,
    pub monthly: Option<HistoricalStatsPeriodGroup>,
}

/// [Bungie documentatio](https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyHistoricalStatsValue.html#schema_Destiny-HistoricalStats-DestinyHistoricalStatsValue)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalStatsValue {
    pub stat_id: String,
    pub basic: HistoricalStatsValuePair,
    pub pga: Option<HistoricalStatsValuePair>,
    pub weighted: Option<HistoricalStatsValuePair>,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "int64_from_str_nullable")]
    pub activity_id: Option<Int64>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyHistoricalStatsValuePair.html#schema_Destiny-HistoricalStats-DestinyHistoricalStatsValuePair)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalStatsValuePair {
    pub value: f64,
    pub display_value: String,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyHistoricalStatsPeriodGroup.html#schema_Destiny-HistoricalStats-DestinyHistoricalStatsPeriodGroup)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalStatsPeriodGroup {
    /// if not set: all time
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub period: Option<APIdateTime>,
    pub activity_details: HistoricalStatsActivity,
    pub values: HashMap<String, HistoricalStatsValue>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyHistoricalStatsActivity.html#schema_Destiny-HistoricalStats-DestinyHistoricalStatsActivity)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalStatsActivity {
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    // TODO: activity
    pub reference_id: Uint32,
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    pub director_activity_hash: Uint32,
    pub instance_id: Int64,
    pub mode: Int32,
    pub modes: Vec<Int32>,
    pub is_private: bool,
    pub membership_type: MembershipType,
}