use {
    crate::{
        models::{
            types::*,
            quests::ObjectiveProgress,
            membership::MembershipType
        },
    },
    serde::Deserialize,
    std::collections::HashMap,
};
/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Entities-Inventory-DestinyInventoryComponent.html#schema_Destiny-Entities-Inventory-DestinyInventoryComponent)
#[derive(Debug, Deserialize)]
pub struct InventoryComponent {
    pub items: Vec<ItemComponent>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Entities-Items-DestinyItemComponent.html#schema_Destiny-Entities-Items-DestinyItemComponent)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemComponent {
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub item_hash: Hash,
    pub item_instance_id: Option<Int64>,
    pub quantity: Int32,
    pub bind_status: Int32,
    pub location: Int32,
    /// mapped to [`InventoryBucket`](crate::models::manifest::ManifestKey::InventoryBucket)
    pub bucket_hash: Hash,
    pub transfer_status: Int32,
    pub lockable: bool,
    pub state: Int32,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub override_style_item_hash: Option<Hash>,
    #[serde(deserialize_with = "from_timestamp")]
    pub expiration_date: APIdateTime,
    pub is_wrapper: bool,
    pub tooltip_notification_indexes: Vec<Int32>,
    pub metric_hash: Option<Hash>,
    pub metric_objective: ObjectiveProgress,
    pub version_number: Option<Int32>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Components-Inventory-DestinyPlatformSilverComponent.html#schema_Destiny-Components-Inventory-DestinyPlatformSilverComponent)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformSilverComponent {
    /// the key corresponds to the MembershipType of the platform that owns the silver
    // TODO: parse this to MembershipType
    pub platform_silver: HashMap<Int32, ItemComponent>
}