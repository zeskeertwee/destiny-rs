use crate::models::{
    api::DestinyAPI,
    manifest::ManifestKey,
    locale::Locale,
};
use tokio_test::block_on;

#[test]
fn test_get_recommended_manifest_path() {
    super::init_log();

    let path = crate::get_recommended_manifest_path().unwrap();
    log::info!("Recommended manifest path: {:?}", path);
}

#[test]
fn manifest_keys() {
    super::init_log();

    let keys = vec![
        ManifestKey::Achievement,
        ManifestKey::Activity,
        ManifestKey::ActivityGraph,
        ManifestKey::ActivityMode,
        ManifestKey::ActivityModifier,
        ManifestKey::ActivityType,
        ManifestKey::Artifact,
        ManifestKey::Bond,
        ManifestKey::BreakerType,
        ManifestKey::Checklist,
        ManifestKey::Class,
        ManifestKey::Collectible,
        ManifestKey::DamageType,
        ManifestKey::Destination,
        ManifestKey::EnergyType,
        ManifestKey::EquipmentSlot,
        ManifestKey::Faction,
        ManifestKey::Gender,

        // TODO: this one appears to not use hashes
        //ManifestKey::HistoricalStats,

        ManifestKey::InventoryBucket,
        ManifestKey::InventoryItem,
        ManifestKey::ItemCategory,
        ManifestKey::ItemTierType,
        ManifestKey::Location,
        ManifestKey::Lore,
        ManifestKey::MaterialRequirementSet,
        ManifestKey::MedalTier,
        ManifestKey::Metric,
        ManifestKey::Milestone,
        ManifestKey::Objective,
        ManifestKey::Place,
        ManifestKey::PlugSet,
        ManifestKey::PowerCap,
        ManifestKey::PresentationNode,
        ManifestKey::Progression,
        ManifestKey::ProgressionLevelRequirement,
        ManifestKey::Race,
        ManifestKey::Record,
        ManifestKey::ReportReasonCategory,
        ManifestKey::RewardSource,
        ManifestKey::SackRewardItemList,
        ManifestKey::SandboxPattern,
        ManifestKey::SandboxPerk,
        ManifestKey::Season,
        ManifestKey::SeasonPass,
        ManifestKey::SocketCategory,
        ManifestKey::SocketType,
        ManifestKey::Stat,
        ManifestKey::StatGroup,
        ManifestKey::TalentGrid,
        ManifestKey::TraitCategory,
        ManifestKey::Trait,
        ManifestKey::Unlock,
        ManifestKey::Vendor,
        ManifestKey::VendorGroup
    ];

    let api = DestinyAPI::new(&std::env::var("BUNGIE_API_KEY").unwrap()).unwrap();
    let mut manifest_path = crate::get_recommended_manifest_path().expect("Unable to get manifest path");
    
    let m = block_on(api.manifest(manifest_path, Locale::English)).unwrap();

    for key in keys {
        log::info!("key: {}: {:?}", key, m.query_raw::<serde_json::Value>(&format!("SELECT * FROM {} LIMIT 1", key)).unwrap());
    }
}