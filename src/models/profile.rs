use {
    crate::{
        models::{
            types::*,
            components::{
                vendor::VendorComponent,
                user::ProfileComponent,
                metrics::MetricsComponent,
                inventory::{
                    InventoryComponent,
                    PlatformSilverComponent,
                },
                character::CharacterComponent,
            }
        },
        traits::component::ComponentID,
    },
    serde::Deserialize,
    std::{
        collections::HashMap,
    },
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Responses-DestinyProfileResponse.html#schema_Destiny-Responses-DestinyProfileResponse)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProfileResponse {
    pub vendor_receipts: Option<ComponentWrapper<VendorComponent>>,
    pub profile_inventory: Option<ComponentWrapper<InventoryComponent>>,
    pub profile_currencies: Option<ComponentWrapper<InventoryComponent>>,
    pub profile: Option<ComponentWrapper<ProfileComponent>>,
    pub platform_silver: Option<ComponentWrapper<PlatformSilverComponent>>,
    pub metrics: Option<ComponentWrapper<MetricsComponent>>,
    pub characters: Option<ComponentWrapper<HashMap<String, CharacterComponent>>>
}

#[derive(Debug, Deserialize)]
pub struct ComponentWrapper<T> {
    pub data: Option<T>,
    pub privacy: Option<Int32>,
    pub disabled: Option<bool>,
}

//#[derive(Debug, Deserialize)]
//pub struct CharacterComponentWrapper {
//    pub data: Option<HashMap<String, CharacterComponent>>,
//    pub privacy: Int32,
//    pub disabled: Option<bool>,
//}

pub enum Component {
    None,
    Profiles,
    VendorReceipts,
    ProfileInventories,
    ProfileCurrencies,
    ProfileProgression,
    PlatformSilver,
    Characters,
    CharacterInventories,
    CharacterProgressions,
    CharacterRenderData,
    CharacterActivities,
    CharacterEquipment,
    ItemInstances,
    ItemObjectives,
    ItemPerks,
    ItemRenderData,
    ItemStats,
    ItemSockets,
    ItemTalentGrids,
    ItemCommonData,
    ItemPlugStates,
    ItemPlugObjectives,
    ItemReusablePlugs,
    Vendors,
    VendorCategories,
    VendorSales,
    Kiosks,
    CurrencyLookups,
    PresentationNodes,
    Collectibles,
    Records,
    Transitory,
    Metrics
}

impl ComponentID for Component {
    fn component_id(&self) -> Int32 {
        match self {
            Component::None => 0,
            Component::Profiles => 100,
            Component::VendorReceipts => 101,
            Component::ProfileInventories => 102,
            Component::ProfileCurrencies => 103,
            Component::ProfileProgression => 104,
            Component::PlatformSilver => 105,
            Component::Characters => 200,
            Component::CharacterInventories => 201,
            Component::CharacterProgressions => 202,
            Component::CharacterRenderData => 203,
            Component::CharacterActivities => 204,
            Component::CharacterEquipment => 205,
            Component::ItemInstances => 300,
            Component::ItemObjectives => 301,
            Component::ItemPerks => 302,
            Component::ItemRenderData => 303,
            Component::ItemStats => 304,
            Component::ItemSockets => 305,
            Component::ItemTalentGrids => 306,
            Component::ItemCommonData => 307,
            Component::ItemPlugStates => 308,
            Component::ItemPlugObjectives => 309,
            Component::ItemReusablePlugs => 310,
            Component::Vendors => 400,
            Component::VendorCategories => 401,
            Component::VendorSales => 402,
            Component::Kiosks => 500,
            Component::CurrencyLookups => 600,
            Component::PresentationNodes => 700,
            Component::Collectibles => 800,
            Component::Records => 900,
            Component::Transitory => 1000,
            Component::Metrics => 1100,
        }
    }
}