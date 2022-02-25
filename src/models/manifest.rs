use {
    crate::{
        models::types::*,
        DOWNLOAD_BASE_URL,
    },
    sqlite::Connection,
    serde::{
        Deserialize,
        de::DeserializeOwned
    },
    std::{
        collections::HashMap,
        path::PathBuf,
        fs::File,
        fmt,
        sync::Mutex,
        io::{
            copy,
            Cursor,
        }
    },
    zip::{
        ZipArchive,
    },
    anyhow::{
        Result,
        anyhow
    },
    reqwest,
};

pub struct Manifest {
    pub version: String,
    pub(crate) database: Mutex<Connection>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Config-GearAssetDataBaseDefinition.html#schema_Destiny-Config-GearAssetDataBaseDefinition)
#[derive(Debug, Deserialize)]
pub struct MobileGearAssetDatabaseDefenition {
    pub version: Int32,
    pub path: String,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Config-DestinyManifest.html#schema_Destiny-Config-DestinyManifest)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDestinyManifestResponse {
    pub version: String,
    pub mobile_asset_content_path: String,
    pub mobile_gear_asset_data_bases: Vec<MobileGearAssetDatabaseDefenition>,
    pub mobile_world_content_paths: HashMap<String, String>,
    pub mobile_clan_banner_database_path: String,
    #[serde(rename = "mobileGearCDN")]
    pub mobile_gear_cdn: HashMap<String, String>,
}

impl Manifest {
    pub(crate) async fn download_database(client: &reqwest::Client, url: &str, path: &PathBuf) -> Result<Connection> {
        let full_url = format!("{}{}", DOWNLOAD_BASE_URL, url);

        let mut save_file = File::create(&path)?;
        
        let bytes = client.get(&full_url)
            .send()
            .await?
            .bytes()
            .await?;

        let mut zip_file = ZipArchive::new(Cursor::new(bytes))?;
        let mut file = zip_file.by_index(0)?;
        copy(&mut file, &mut save_file)?;

        Ok(sqlite::Connection::open(&path)?)
    }

    pub fn query_raw<T: DeserializeOwned>(&self, q: &str) -> Result<Vec<T>> {
        let lock = match self.database.lock() {
            Ok(x) => x,
            Err(_) => return Err(anyhow!("Unable to lock connection mutex"))
        };
        
        let mut cursor = lock.prepare(q)?.cursor();
        cursor.bind(&[])?;
        let mut data = Vec::new();
        while let Some(row) = cursor.next()? {
            data.push(serde_json::from_str(match row[0].as_string() {
                Some(x) => x,
                None => continue
            })?);
        }

        Ok(data)
    }

    pub fn query<T: DeserializeOwned + Clone>(&self, hash: Hash, table: ManifestKey) -> Result<T> {
        let query = {
            if hash > i32::MAX as u32 {
                format!("SELECT json FROM {} WHERE id + 4294967296 = {}", table, hash)
            } else {
                format!("SELECT json FROM {} WHERE id = {}", table, hash)
            }
        };

        let result: Vec<T> = self.query_raw(&query)?;
        match result.len() {
            0 => Err(anyhow!("Invalid hash, no match found!")),
            1 => Ok(result[0].clone()),
            _ => Err(anyhow!("Multiple entries for hash found!"))
        }
    }
}

pub enum ManifestKey {
    Achievement,
    Activity,
    ActivityGraph,
    ActivityMode,
    ActivityModifier,
    ActivityType,
    Artifact,
    Bond,
    BreakerType,
    Checklist,
    Class,
    Collectible,
    DamageType,
    Destination,
    EnemyRace,
    EnergyType,
    EquipmentSlot,
    Faction,
    Gender,
    HistoricalStats,
    InventoryBucket,
    InventoryItem,
    ItemCategory,
    ItemTierType,
    Location,
    Lore,
    MaterialRequirementSet,
    MedalTier,
    Metric,
    Milestone,
    Objective,
    Place,
    PlugSet,
    PowerCap,
    PresentationNode,
    Progression,
    ProgressionLevelRequirement,
    Race,
    Record,
    ReportReasonCategory,
    RewardSource,
    SackRewardItemList,
    SandboxPattern,
    SandboxPerk,
    Season,
    SeasonPass,
    SocketCategory,
    SocketType,
    Stat,
    StatGroup,
    TalentGrid,
    TraitCategory,
    Trait,
    Unlock,
    Vendor,
    VendorGroup
}

impl fmt::Display for ManifestKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tmp = match self {
            ManifestKey::Achievement => "DestinyAchievementDefinition",
            ManifestKey::Activity => "DestinyActivityDefinition",
            ManifestKey::ActivityGraph => "DestinyActivityGraphDefinition",
            ManifestKey::ActivityMode => "DestinyActivityModeDefinition",
            ManifestKey::ActivityModifier => "DestinyActivityModifierDefinition",
            ManifestKey::ActivityType => "DestinyActivityTypeDefinition",
            ManifestKey::Artifact => "DestinyArtifactDefinition",
            ManifestKey::Bond => "DestinyBondDefinition",
            ManifestKey::BreakerType => "DestinyBreakerTypeDefinition",
            ManifestKey::Checklist => "DestinyChecklistDefinition",
            ManifestKey::Class => "DestinyClassDefinition",
            ManifestKey::Collectible => "DestinyCollectibleDefinition",
            ManifestKey::DamageType => "DestinyDamageTypeDefinition",
            ManifestKey::Destination => "DestinyDestinationDefinition",
            ManifestKey::EnemyRace => "DestinyEnemyRaceDefinition",
            ManifestKey::EnergyType => "DestinyEnergyTypeDefinition",
            ManifestKey::EquipmentSlot => "DestinyEquipmentSlotDefinition",
            ManifestKey::Faction => "DestinyFactionDefinition",
            ManifestKey::Gender => "DestinyGenderDefinition",
            ManifestKey::HistoricalStats => "DestinyHistoricalStatsDefinition",
            ManifestKey::InventoryBucket => "DestinyInventoryBucketDefinition",
            ManifestKey::InventoryItem => "DestinyInventoryItemDefinition",
            ManifestKey::ItemCategory => "DestinyItemCategoryDefinition",
            ManifestKey::ItemTierType => "DestinyItemTierTypeDefinition",
            ManifestKey::Location => "DestinyLocationDefinition",
            ManifestKey::Lore => "DestinyLoreDefinition",
            ManifestKey::MaterialRequirementSet => "DestinyMaterialRequirementSetDefinition",
            ManifestKey::MedalTier => "DestinyMedalTierDefinition",
            ManifestKey::Metric => "DestinyMetricDefinition",
            ManifestKey::Milestone => "DestinyMilestoneDefinition",
            ManifestKey::Objective => "DestinyObjectiveDefinition",
            ManifestKey::Place => "DestinyPlaceDefinition",
            ManifestKey::PlugSet => "DestinyPlugSetDefinition",
            ManifestKey::PowerCap => "DestinyPowerCapDefinition",
            ManifestKey::PresentationNode => "DestinyPresentationNodeDefinition",
            ManifestKey::Progression => "DestinyProgressionDefinition",
            ManifestKey::ProgressionLevelRequirement => "DestinyProgressionLevelRequirementDefinition",
            ManifestKey::Race => "DestinyRaceDefinition",
            ManifestKey::Record => "DestinyRecordDefinition",
            ManifestKey::ReportReasonCategory => "DestinyReportReasonCategoryDefinition",
            ManifestKey::RewardSource => "DestinyRewardSourceDefinition",
            ManifestKey::SackRewardItemList => "DestinySackRewardItemListDefinition",
            ManifestKey::SandboxPattern => "DestinySandboxPatternDefinition",
            ManifestKey::SandboxPerk => "DestinySandboxPerkDefinition",
            ManifestKey::Season => "DestinySeasonDefinition",
            ManifestKey::SeasonPass => "DestinySeasonPassDefinition",
            ManifestKey::SocketCategory => "DestinySocketCategoryDefinition",
            ManifestKey::SocketType => "DestinySocketTypeDefinition",
            ManifestKey::Stat => "DestinyStatDefinition",
            ManifestKey::StatGroup => "DestinyStatGroupDefinition",
            ManifestKey::TalentGrid => "DestinyTalentGridDefinition",
            ManifestKey::TraitCategory => "DestinyTraitCategoryDefinition",
            ManifestKey::Trait => "DestinyTraitDefinition",
            ManifestKey::Unlock => "DestinyUnlockDefinition",
            ManifestKey::Vendor => "DestinyVendorDefinition",
            ManifestKey::VendorGroup => "DestinyVendorGroupDefinition",
        };

        write!(f, "{}", tmp)
    }
}