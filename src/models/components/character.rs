use {
    crate::{
        models::{
            types::*,
            membership::MembershipType,
            color::Color,
            progression::Progression,
        },
        traits::id::{
            DestinyMembershipID,
            CharacterID
        }
    },
    serde::Deserialize,
};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterComponent {
    #[serde(deserialize_with = "int64_from_str")]
    pub membership_id: Int64,
    pub membership_type: MembershipType,
    #[serde(deserialize_with = "int64_from_str")]
    pub character_id: Int64,
    #[serde(deserialize_with = "from_timestamp")]
    pub date_last_played: APIdateTime,
    #[serde(deserialize_with = "int64_from_str")]
    pub minutes_played_this_session: Int64,
    #[serde(deserialize_with = "int64_from_str")]
    pub minutes_played_total: Int64,
    pub light: Int32,
    pub stats: CharacterStats,
    /// mapped to [`Race`](crate::models::manifest::ManifestKey::Race)
    pub race_hash: Hash,
    /// mapped to [`Gender`](crate::models::manifest::ManifestKey::Gender)
    pub gender_hash: Hash,
    /// mapped to [`Class`](crate::models::manifest::ManifestKey::Class)
    pub class_hash: Hash,
    pub race_type: Int32,
    pub class_type: Int32,
    pub gender_type: Int32,
    pub emblem_path: String,
    pub emblem_background_path: String,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub emblem_hash: Hash,
    pub emblem_color: Color,
    pub level_progression: Progression,
    pub base_character_level: Int32,
    pub percent_to_next_level: f32,
    /// mapped to [`Record`](crate::models::manifest::ManifestKey::Record)
    pub title_record_hash: Option<Hash>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterStats {
    #[serde(rename = "1935470627")]
    pub power: Int32,
    #[serde(rename = "2996146975")]
    pub mobility: Int32,
    #[serde(rename = "392767087")]
    pub resilience: Int32,
    #[serde(rename = "1943323491")]
    pub recovery: Int32,
    #[serde(rename = "1735777505")]
    pub discipline: Int32,
    #[serde(rename = "144602215")]
    pub intellect: Int32,
    #[serde(rename = "4244567218")]
    pub strength: Int32,
}

impl DestinyMembershipID for CharacterComponent {
    fn destiny_membership_id(&self) -> Int64 {
        self.membership_id
    }
}

impl CharacterID for CharacterComponent {
    fn character_id(&self) -> Int64 {
        self.character_id
    }
}