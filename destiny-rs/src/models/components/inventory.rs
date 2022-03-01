use {
    crate::{
        models::{
            types::*,
            quests::ObjectiveProgress,
        },
    },
    serde::Deserialize,
    std::collections::HashMap,
};
use crate::models::manifest_models::InventoryItem;

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
    // TODO
    pub item_hash: Uint32,
    pub item_instance_id: Option<Int64>,
    pub quantity: Int32,
    pub bind_status: Int32,
    pub location: Int32,
    /// mapped to [`InventoryBucket`](crate::models::manifest::ManifestKey::InventoryBucket)
    // TODO
    pub bucket_hash: Uint32,
    pub transfer_status: Int32,
    pub lockable: bool,
    pub state: Int32,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub override_style_item_hash: Option<Hash<InventoryItem>>,
    #[serde(deserialize_with = "from_timestamp")]
    pub expiration_date: APIdateTime,
    pub is_wrapper: bool,
    pub tooltip_notification_indexes: Vec<Int32>,
    /// mapped to [`Metric`](crate::models::manifest::ManifestKey::Metric)
    // TODO: metric
    pub metric_hash: Option<Uint32>,
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