use {
    crate::{
        models::{
            types::*,
            groupsv2::GroupUserInfoCard,
            membership::MembershipType,
            ignores::IgnoreResponse,
        },
        traits::id::{
            BNGMembershipID,
            DestinyMembershipID,
            PlatformType,
        }
    },
    serde::Deserialize,
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_User-GeneralUser.html#schema_User-GeneralUser)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralUser {
    #[serde(deserialize_with = "int64_from_str")]
    pub membership_id: Int64,
    pub unique_name: String,
    pub normalized_name: Option<String>,
    pub display_name: String,
    pub profile_picture: Int32,
    pub profile_theme: Int32,
    pub user_title: Int32,
    #[serde(deserialize_with = "int64_from_str")]
    pub success_message_flags: Int64,
    pub is_deleted: bool,
    pub about: String,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub first_access: Option<APIdateTime>,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub last_update: Option<APIdateTime>,
    pub legacy_portal_uid: Option<Int64>,
    pub context: Option<UserToUserContext>,
    pub psn_display_name: Option<String>,
    pub xbox_display_name: Option<String>,
    pub fb_display_name: Option<String>,
    pub show_activity: Option<bool>,
    pub locale: String,
    pub locale_inherit_default: bool,
    pub last_ban_report_id: Option<Int64>,
    pub show_group_messaging: bool,
    pub profile_picture_path: String,
    pub profile_picture_wide_path: Option<String>,
    pub profile_theme_name: String,
    pub user_title_display: String,
    pub status_text: String,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub status_date: Option<APIdateTime>,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub profile_ban_expire: Option<APIdateTime>,
    pub blizzard_display_name: Option<String>,
    pub steam_display_name: Option<String>,
    pub stadia_display_name: Option<String>,
    pub twitch_display_name: Option<String>,
}

impl BNGMembershipID for GeneralUser {
    fn bng_membership_id(&self) -> Int64 {
        self.membership_id
    }
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_User-HardLinkedUserMembership.html#schema_User-HardLinkedUserMembership)
#[derive(Debug, Deserialize)]
pub struct HardLinkedUserMembership {
    #[serde(rename = "membershipType")]
    pub membership_type: MembershipType,
    #[serde(deserialize_with = "int64_from_str")]
    #[serde(rename = "membershipId")]
    pub membership_id: Int64,
    #[serde(rename = "CrossSaveOverriddenType")]
    pub cross_save_overridden_type: MembershipType,
    #[serde(rename = "CrossSaveOverriddenMembershipId")]
    pub cross_save_overridden_membership_id: Option<Int64>,
}

impl DestinyMembershipID for HardLinkedUserMembership {
    fn destiny_membership_id(&self) -> Int64 {
        self.membership_id
    }
}

impl PlatformType for HardLinkedUserMembership {
    fn platform_type(&self) -> MembershipType {
        self.membership_type
    }
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_User-UserMembershipData.html#schema_User-UserMembershipData)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMembershipData {
    pub destiny_memberships: Vec<GroupUserInfoCard>,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "int64_from_str_nullable")]
    pub primary_membership_id: Option<Int64>,
    pub bungie_net_user: GeneralUser,
}

impl DestinyMembershipID for UserMembershipData {
    fn destiny_membership_id(&self) -> Int64 {
        if let Some(x) = self.primary_membership_id {
            x
        } else {
            self.destiny_memberships[0].membership_id
        }
    }
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_User-UserToUserContext.html#schema_User-UserToUserContext)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserToUserContext {
    pub is_following: bool,
    pub ignore_status: IgnoreResponse,
    #[serde(default = "serde_none")]
    #[serde(deserialize_with = "from_timestamp_nullable")]
    pub global_ignore_end_date: Option<APIdateTime>,
}