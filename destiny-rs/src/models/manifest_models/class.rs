use {
    crate::models::{
        types::*,
    },
    serde::Deserialize,
};
use crate::models::manifest::ManifestKey;
use crate::traits::manifest_key::ManifestTableKey;

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyClassDefinition.html#schema_Destiny-Definitions-DestinyGenderDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub class_type: Int32,
    pub display_properties: ClassDisplayProperties,
    pub hash: Hash<Self>,
    pub index: Int32,
    pub redacted: bool,
}

impl ManifestTableKey for Class {
    const TABLE_KEY: ManifestKey = ManifestKey::Class;
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassDisplayProperties {
    pub name: String,
    pub has_icon: bool,
}