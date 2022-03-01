use {
    crate::{
        models::{
            types::*,
            groupsv2::UserInfoCard,
        },
        traits::id::DestinyMembershipID,
    },
    serde::Deserialize,
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Entities-Profiles-DestinyProfileComponent.html#schema_Destiny-Entities-Profiles-DestinyProfileComponent)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileComponent {
    pub user_info: UserInfoCard,
    #[serde(deserialize_with = "from_timestamp")]
    pub date_last_played: APIdateTime,
    pub versions_owned: Int32,
    // TODO: parse to int64
    pub character_ids: Vec<String>,
    pub season_hashes: Vec<Hash>,
    pub current_season_hash: Option<Hash>,
    pub current_season_reward_power_cap: Option<Int32>,
}

impl DestinyMembershipID for ProfileComponent {
    fn destiny_membership_id(&self) -> Int64 {
        self.user_info.membership_id
    }
}