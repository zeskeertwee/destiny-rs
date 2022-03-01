use {
    crate::models::{
        types::*,
        manifest_models::Vendor,
    },
    serde::Deserialize,
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Components-Kiosks-DestinyKiosksComponent.html#schema_Destiny-Components-Kiosks-DestinyKiosksComponent)
pub struct KiosksComponent {
    pub kiosk_items: HashMap<Hash, Vendor>
}