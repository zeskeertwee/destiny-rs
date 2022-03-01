use {
    crate::{
        models::{
            types::*,
            membership::MembershipType,
        },
        traits::id::{
            DestinyMembershipID,
            PlatformType,
        }
    },
    serde::Deserialize,
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_GroupsV2-GroupUserInfoCard.html#schema_GroupsV2-GroupUserInfoCard)
#[derive(Debug, Deserialize)]
pub struct GroupUserInfoCard {
    #[serde(rename = "LastSeenDisplayName")]
    pub last_seen_display_name: String,
    #[serde(rename = "LastSeenDisplayNameType")]
    pub last_seen_display_name_type: MembershipType,
    #[serde(rename = "supplementalDisplayName")]
    pub supplemental_display_name: Option<String>,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "crossSaveOverride")]
    pub cross_save_override: MembershipType,
    #[serde(rename = "applicableMembershipTypes")]
    pub applicable_membership_types: Vec<MembershipType>,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "membershipType")]
    pub membership_type: MembershipType,
    #[serde(deserialize_with = "int64_from_str")]
    #[serde(rename = "membershipId")]
    pub membership_id: Int64,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl DestinyMembershipID for GroupUserInfoCard {
    fn destiny_membership_id(&self) -> Int64 {
        self.membership_id
    }
}

impl PlatformType for GroupUserInfoCard {
    fn platform_type(&self) -> MembershipType {
        self.membership_type
    }
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_User-UserInfoCard.html#schema_User-UserInfoCard)
#[derive(Debug, Deserialize)]
pub struct UserInfoCard {
    #[serde(rename = "supplementalDisplayName")]
    pub supplemental_display_name: Option<String>,
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,
    #[serde(rename = "crossSaveOverride")]
    pub cross_save_override: MembershipType,
    #[serde(rename = "applicableMembershipTypes")]
    pub applicable_membership_types: Option<Vec<MembershipType>>,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "membershipType")]
    pub membership_type: MembershipType,
    #[serde(deserialize_with = "int64_from_str")]
    #[serde(rename = "membershipId")]
    pub membership_id: Int64,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl DestinyMembershipID for UserInfoCard {
    fn destiny_membership_id(&self) -> Int64 {
        self.membership_id
    }
}

impl PlatformType for UserInfoCard {
    fn platform_type(&self) -> MembershipType {
        self.membership_type
    }
}