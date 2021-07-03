use {
    serde::Deserialize,
};

pub mod season;
pub mod presentation_node;
pub mod inventory_item;
pub mod gender;
pub mod race;
pub mod class;
pub mod lore;
pub mod record;
//pub mod vendor;

// re-exports
pub use {
    season::Season,
    presentation_node::PresentationNode,
    inventory_item::InventoryItem,
    gender::Gender,
    race::Race,
    class::Class,
    lore::Lore,
    record::Record,
};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayProperties {
    pub name: String,
    pub description: String,
    pub has_icon: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Common-DestinyDisplayPropertiesDefinition.html#schema_Destiny-Definitions-Common-DestinyDisplayPropertiesDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDisplayProperties {
    pub description: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_sequences: Option<Vec<IconSequence>>,
    pub high_res_icon: Option<String>,
    pub has_ison: Option<bool>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Common-DestinyIconSequenceDefinition.html#schema_Destiny-Definitions-Common-DestinyIconSequenceDefinition)
#[derive(Debug, Deserialize, Clone)]
pub struct IconSequence {
    pub frames: Vec<String>,
}