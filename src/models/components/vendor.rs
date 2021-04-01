use {
    crate::{
        models::{
            types::*,
            item::DestinyItemQuantity,
        }
    },
    serde::Deserialize,
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Entities-Profiles-DestinyVendorReceiptsComponent.html#schema_Destiny-Entities-Profiles-DestinyVendorReceiptsComponent)
#[derive(Debug, Deserialize)]
pub struct VendorComponent {
    pub receipts: Vec<VendorReceipt>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Vendors-DestinyVendorReceipt.html#schema_Destiny-Vendors-DestinyVendorReceipt)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VendorReceipt {
    pub currency_paid: DestinyItemQuantity,
    pub item_received: DestinyItemQuantity,
    pub licence_unlock_hash: Hash,
    pub purchased_by_character_id: Int64,
    pub refund_policy: Int32,
    pub sequence_number: Int32,
    pub time_to_expiration: Int64,
    #[serde(deserialize_with = "from_timestamp")]
    pub expires_on: APIdateTime,
}