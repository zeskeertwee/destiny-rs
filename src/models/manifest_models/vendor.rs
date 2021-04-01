use {
    crate::models::{
        types::*,
        manifest_models::{
            DisplayProperties,
            DisplayCategory
        },
    },
    serde::Deserialize
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyVendorDefinition.html#schema_Destiny-Definitions-DestinyVendorDefinition)
pub struct Vendor {
    pub display_properties: DisplayProperties,
    pub vendor_progression_type: Int32,
    pub buy_string: String,
    pub sell_string: String,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub display_item_hash: Hash,
    pub inhibit_buying: bool,
    pub inhibit_selling: bool,
    /// mapped to [`Faction`](crate::models::manifest::ManifestKey::Faction)
    pub faction_hash: Hash,
    pub reset_inverval_minutes: Int32,
    pub reset_offset_minutes: Int32,
    pub failure_strings: Vec<String>,
    pub unlock_ranges: Vec<APIdateRange>,
    pub vendor_identifier: Vec<String>,
    pub vendor_portrait: String,
    pub vendor_banner: String,
    pub enabled: bool,
    pub visible: bool,
    pub vendor_sub_category_identifier: String,
    pub consolidate_categories: bool,
    pub actions: Vec<VendorAction>,
    pub categories: Vec<VendorCategoryEntry>,
    pub original_categories: Vec<VendorCategoryEntry>,
    pub display_categories: Vec<DisplayCategory>,

}

pub struct VendorAction {
    pub description: String,
    pub execute_seconds: Int32,
    pub icon: String,
    pub name: String,
    pub verb: String,
    pub is_positive: bool,
    pub action_id: String,
    pub action_hash: Hash,
    pub auto_preform_action: bool,
}

/// [`Bungie documentation`](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyVendorCategoryEntryDefinition.html#schema_Destiny-Definitions-DestinyVendorCategoryEntryDefinition)
pub struct VendorCategoryEntry {
    pub category_index: Int32,
    pub sort_value: Int32,
    pub category_hash: Hash,
    pub quantity_availible: Int32,
    pub show_unavailible_items: bool,
    pub hide_if_no_currency: bool,
    pub hide_from_regular_purchase: bool,
    pub buy_string_override: String,
    pub disabled_description: String,
    pub display_title: String,
    pub overlay: VendorCategoryOverlay,
    pub vendor_item_indexes: Vec<Int32>,
    pub is_preview: bool,
    pub is_display_only: bool,
    pub reset_inverval_minutes_override: Int32,
    pub reset_offset_minutes_override: Int32,
}

pub struct VendorCategoryOverlay {
    pub choice_description: String,
    pub description: String,
    pub icon: String,
    pub title: String,
    pub currency_item_hash: Option<Hash>
}