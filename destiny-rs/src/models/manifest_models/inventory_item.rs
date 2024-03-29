use {
    crate::{
        models::{
            types::*,
            color::Color,
            manifest_models::DestinyDisplayProperties,
        },
    },
    serde::Deserialize
};
use crate::models::manifest::ManifestKey;
use crate::models::manifest_models::Season;
use crate::traits::manifest_key::ManifestTableKey;

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyInventoryItemDefinition.html#schema_Destiny-Definitions-DestinyInventoryItemDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    // TODO: a lot of properties
    pub display_properties: DestinyDisplayProperties,
    pub tooltip_notifications: Vec<TooltipNotification>,
    /// mapped to [`Collectible`](crate::models::manifest::ManifestKey::Collectible)
    // TODO: collectible
    pub collectible_hash: Option<Uint32>,
    pub icon_watermark: Option<String>,
    pub icon_watermark_shelved: Option<String>,
    pub secondary_icon: String,
    pub secondary_overlay: String,
    pub secondary_special: String,
    pub background_color: Color,
    pub screenshot: Option<String>,
    pub item_type_display_name: String,
    pub flavor_text: String,
    pub ui_item_display_style: String,
    pub item_type_and_tier_display_name: String,
    pub display_source: String,
    pub tooltip_style: Option<String>,
    pub action: ItemActionBlock,
    pub inventory: ItemInventoryBlock,
    pub set_data: Option<ItemSetBlock>,
    /// mapped to [`Lore`](crate::models::manifest::ManifestKey::Lore)
    // TODO: lore
    pub lore_hash: Option<Uint32>,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub summary_item_hash: Option<Hash<InventoryItem>>,
    /// mapped to [`ItemCategory`](crate::models::manifest::ManifestKey::ItemCategory)
    // TODO: item_category
    pub item_category_hashes: Vec<Uint32>,
    /// mapped to [`BreakerType`](crate::models::manifest::ManifestKey::BreakerType)
    // TODO: breaker_type
    pub breaker_type_hash: Option<Uint32>,
    /// mapped to [`DamageType`](crate::models::manifest::ManifestKey::DamageType)
    // TODO: damage_type
    pub damage_type_hashes: Option<Vec<Uint32>>,
    /// mapped to [`DamageType`](crate::models::manifest::ManifestKey::DamageType)
    pub default_damage_type_hash: Option<Uint32>,
    /// mapped to [`Season`](crate::models::manifest::ManifestKey::Season)
    pub season_hash: Option<Hash<Season>>,
    pub hash: Hash<Self>,
    pub redacted: bool,
}

impl ManifestTableKey for InventoryItem {
    const TABLE_KEY: ManifestKey = ManifestKey::InventoryItem;
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemTooltipNotification.html#schema_Destiny-Definitions-DestinyItemTooltipNotification)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TooltipNotification {
    pub display_string: String,
    pub display_style: String,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemActionBlockDefinition.html#schema_Destiny-Definitions-DestinyItemActionBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemActionBlock {
    pub verb_name: String,
    pub verb_description: String,
    pub is_positive: bool,
    pub overlay_screen_name: Option<String>,
    pub overlay_icon: Option<String>,
    pub required_cooldown_seconds: Int32,
    pub required_items: Vec<ItemActionRequiredItem>,
    pub progression_rewards: Vec<ProgressionReward>,
    pub action_type_label: Option<String>,
    pub required_location: Option<String>,
    pub required_cooldown_hash: Uint32,
    pub delete_on_action: bool,
    pub consume_entire_stack: bool,
    pub use_on_acquire: bool,
}


/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemActionRequiredItemDefinition.html#schema_Destiny-Definitions-DestinyItemActionRequiredItemDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemActionRequiredItem {
    pub count: Int32,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub item_hash: Hash<InventoryItem>,
    pub delete_on_action: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyProgressionRewardDefinition.html#schema_Destiny-Definitions-DestinyProgressionRewardDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionReward {
    /// mapped to [`Progression`](crate::models::manifest::ManifestKey::Progression)
    // TODO: progression
    pub progression_hash: Uint32,
    pub amount: Int32,
    pub apply_throttles: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemInventoryBlockDefinition.html#schema_Destiny-Definitions-DestinyItemInventoryBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemInventoryBlock {
    pub stack_unique_label: String,
    pub max_stack_size: Int32,
    /// mapped to [`InventoryBucket`](crate::models::manifest::ManifestKey::InventoryBucket)
    // TODO: inventory_bucket
    pub bucket_type_hash: Uint32,
    /// mapped to [`InventoryBucket`](crate::models::manifest::ManifestKey::InventoryBucket)
    pub recovery_bucket_type_hash: Uint32,
    /// mapped to [`ItemTierType`](crate::models::manifest::ManifestKey::ItemTierType)
    // TODO: item_tier_type
    pub tier_type_hash: Uint32,
    pub is_instance_item: bool,
    pub tier_type_name: String,
    pub tier_type: Int32,
    pub expiration_tooltip: String,
    pub expired_in_activity_message: String,
    pub expired_in_orbit_message: String,
    pub suppress_expiration_when_objectives_complete: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemSetBlockDefinition.html#schema_Destiny-Definitions-DestinyItemSetBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemSetBlock {
    pub item_list: Vec<ItemSetBlockEntry>,
    pub require_ordered_set_item_add: bool,
    pub set_is_featured: bool,
    pub set_type: String,
    pub quest_line_name: String,
    pub quest_line_description: String,
    pub quest_step_summary: String,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemSetBlockEntryDefinition.html#schema_Destiny-Definitions-DestinyItemSetBlockEntryDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemSetBlockEntry {
    pub tracking_value: Int32,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub item_hash: Hash<InventoryItem>,
}