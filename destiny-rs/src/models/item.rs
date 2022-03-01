use {
    crate::{
        models::{
            types::*,
        },
    },
    serde::Deserialize,
};
use crate::models::manifest_models::InventoryItem;

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-DestinyItemQuantity.html#schema_Destiny-DestinyItemQuantity)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemQuantity {
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    item_hash: Hash<InventoryItem>,
    item_instance_id: Option<Int64>,
    quantity: Int32,
}