use {
    crate::{
        API_BASE_URL,
        models::{
            manifest::{
                Manifest,
                GetDestinyManifestResponse,
            },
            activity_mode::ActivityMode,
            response::GeneralAPIResponse,
            public_milestone::PublicMilestone,
            user::{
                GeneralUser,
                HardLinkedUserMembership,
                UserMembershipData,
            },
            profile::DestinyProfileResponse,
            groupsv2::UserInfoCard,
            membership::MembershipType,
            locale::Locale,
            historical_stats::HistoricalStatsByPeriod,
        },
        traits::{
            id::{
                BNGMembershipID,
                DestinyMembershipID,
                PlatformType,
                CharacterID
            },
            component::ComponentID,
        }
    },
    std::{
        string::ToString,
        path::PathBuf,
        collections::HashMap,
        sync::Mutex,
        fs::{
            File,
            DirEntry,
            read_dir,
            read_to_string,
        },
        io::{
            self,
            Write,
            Read,
        },
    },
    anyhow::{
        Result,
        anyhow,
    },
    serde::{
        Deserialize,
        Serialize,
        de::DeserializeOwned,
    },
    serde_json,
    reqwest,
};

pub struct DestinyAPI {
    client: reqwest::Client,
}

#[derive(Serialize, Deserialize)]
struct ManifestDownloadVersion {
    locales: HashMap<Locale, DownloadedDatabase>,
}

#[derive(Serialize, Deserialize)]
struct DownloadedDatabase {
    version: String,
    path: PathBuf,
}

impl DestinyAPI {
    pub fn new(key: &str) -> Result<DestinyAPI> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-API-Key",
            reqwest::header::HeaderValue::from_str(key)?,
        );
        Ok(DestinyAPI {
            client: reqwest::Client::builder().default_headers(headers).build()?,
        })
    }

    pub async fn get_user_by_bungie_net_id<T: BNGMembershipID>(&self, id: &T) -> Result<GeneralUser> {
        Ok(self.get_request(&format!("User/GetBungieNetUserById/{}/", id.bng_membership_id())).await?.response)
    }

    pub async fn get_user_by_name(&self, username: String) -> Result<Vec<GeneralUser>> {
        Ok(self.get_request::<Vec<GeneralUser>>(&format!("User/SearchUsers?q={}/", username)).await?.response)
    }

    pub async fn get_user_by_steamid64<T: ToString>(&self, steamid: &T) -> Result<UserMembershipData> {
        let hardlinked = self.get_request::<HardLinkedUserMembership>(&format!("User/GetMembershipFromHardLinkedCredential/SteamID/{}/", steamid.to_string())).await?.response;
        Ok(self.get_user_membership_data_by_membershipid_destiny(&hardlinked, &hardlinked).await?)
    }

    pub async fn get_user_membership_data_by_membershipid_destiny<U: DestinyMembershipID, P: PlatformType>(&self, user: &U, platform: &P) -> Result<UserMembershipData> {
        Ok(self.get_request(&format!("User/GetMembershipsById/{}/{}/", user.destiny_membership_id(), platform.platform_type().into_i32())).await?.response)
    }

    pub async fn get_user_membership_data_by_membershipid_bng<U: BNGMembershipID, P: PlatformType>(&self, user: &U, platform: &P) -> Result<UserMembershipData> {
        Ok(self.get_request(&format!("User/GetMembershipsById/{}/{}/", user.bng_membership_id(), platform.platform_type().into_i32())).await?.response)
    }

    /// platform must not be [`BungieNet`](crate::models::membership::MembershipType::BungieNet)
    pub async fn get_destiny_player_by_name<T: ToString>(&self, username: &T, platform: MembershipType) -> Result<Vec<UserInfoCard>> {
        Ok(self.get_request(&format!("Destiny2/SearchDestinyPlayer/{}/{}/", platform.into_i32(), username.to_string())).await?.response)
    }

    /// platform must not be [`BungieNet`](crate::models::membership::MembershipType::BungieNet)
    pub async fn get_components<U: DestinyMembershipID, P: PlatformType, C: ComponentID>(&self, user: &U, platform: &P, components: Vec<C>) -> Result<DestinyProfileResponse> {
        let mut url = format!("Destiny2/{}/Profile/{}/?components=", platform.platform_type().into_i32(), user.destiny_membership_id());
        for component in components {
            url.push_str(&format!("{},", component.component_id()));
        }
        Ok(self.get_request(&url).await?.response)
    }

    pub async fn get_public_milestones(&self) -> Result<HashMap<String, PublicMilestone>> {
        Ok(self.get_request("Destiny2/Milestones/").await?.response)
    }

    pub async fn get_historical_stats<P: PlatformType, U: DestinyMembershipID, C: CharacterID>(&self, platform: &P, user: &U, character: &C, gamemodes: Option<Vec<ActivityMode>>) -> Result<HashMap<String, HistoricalStatsByPeriod>> {
        let mut url = format!("Destiny2/{}/Account/{}/Character/{}/Stats/", platform.platform_type().into_i32(), user.destiny_membership_id(), character.character_id());
        if let Some(x) = gamemodes {
            url.push_str("?modes=");
            for i in x {
                url.push_str(&format!("{},", i.to_int32()));
            }
        }
        Ok(self.get_request(&url).await?.response)
    }

    pub async fn get_request<T: DeserializeOwned>(&self, url: &str) -> Result<GeneralAPIResponse<T>> {
        if cfg!(debug_assertions) {
            println!("API_CALL: {}/{}", API_BASE_URL, url);
        }
        let raw_response = self.client.get(&format!("{}/{}", API_BASE_URL, url))
            .send()
            .await?
            .text()
            .await?;
        //println!("{:?}", raw_response);
        Ok(serde_json::from_str(&raw_response)?)
    }

    pub async fn manifest(&self, p: PathBuf, loc: Locale) -> Result<Manifest> {
        let files: Vec<io::Result<DirEntry>> = read_dir(&p)?.collect();

        let res = self.call_get_manifest().await?.response;

        for f in files {
            if f?.file_name() == "manifestinfo.json" {
                let mut manifest_info_path = p.clone();
                manifest_info_path.push("manifestinfo.json");
                let data: ManifestDownloadVersion = serde_json::from_str(&read_to_string(manifest_info_path)?)?;
                
                if let Some(x) = data.locales.get(&loc) {
                    if x.version == res.version {
                        return Ok(Manifest {
                            database: Mutex::new(sqlite::Connection::open(&x.path)?),
                            version: x.version.to_owned()
                        });
                    }
                }
            }
        }
        
        Ok(self.download_manifest(res, p, loc).await?)
    }

    pub async fn manifest_up_to_date(&self, p: PathBuf, loc: Locale) -> Result<bool> {
        let files: Vec<io::Result<DirEntry>> = read_dir(&p)?.collect();

        let res = self.call_get_manifest().await?.response;

        for f in files {
            if f?.file_name() == "manifestinfo.json" {
                let mut manifest_info_path = p.clone();
                manifest_info_path.push("manifestinfo.json");
                let data: ManifestDownloadVersion = serde_json::from_str(&read_to_string(manifest_info_path)?)?;
                
                if let Some(x) = data.locales.get(&loc) {
                    if x.version == res.version {
                        return Ok(true);
                    } else {
                        return Ok(false)
                    }
                }
            }
        }

        return Err(anyhow!("Manifest not found"))
    }

    pub async fn manifest_unchecked(&self, p: PathBuf) -> Result<Manifest> {
        Ok(Manifest {
            database: Mutex::new(sqlite::Connection::open(&p)?),
            version: "".to_string()
        })
    }

    async fn call_get_manifest(&self) -> Result<GeneralAPIResponse<GetDestinyManifestResponse>> {
        self.get_request::<GetDestinyManifestResponse>("Destiny2/Manifest/").await
    }

    pub(crate) async fn download_manifest(&self, response: GetDestinyManifestResponse, p: PathBuf, loc: Locale) -> Result<Manifest> {
        // we are only interested in the MobileWorldContent database, since the other two don't contain relevant data
        // MobileGearAssets contains data to render 3d models (my guess is that the mobile app uses this one to render the 3d models)
        // MobileAssets is empty
        // MobileWorldContent contains the defenitions of hashes <- what we are interested in

        let mut mobile_world_content_file_path = p.clone();
        mobile_world_content_file_path.push(format!("{}_{}.sqlite", response.version, loc));
        let mobile_world_content_url = match response.mobile_world_content_paths.get(&format!("{}", loc)) {
            Some(x) => x,
            None => return Err(anyhow!("Locale not found in mobileWorldContentPaths")),
        };

        let mobile_world = Manifest::download_database(&self.client, mobile_world_content_url, &mobile_world_content_file_path).await?;

        let mut version_file_path = p.clone();
        version_file_path.push("manifestinfo.json");

        let mut version_file_data = {
            if let Ok(mut x) = File::open(&version_file_path) {
                let mut content = String::new();
                x.read_to_string(&mut content)?;
                serde_json::from_str(&content)?
            } else {
                ManifestDownloadVersion { locales: HashMap::new() }
            }
        };

        version_file_data.locales.insert(loc, DownloadedDatabase { version: response.version.clone(), path: mobile_world_content_file_path });

        let mut version_file = File::create(&version_file_path)?;
        version_file.write_all(serde_json::to_string(&version_file_data)?.as_bytes())?;

        Ok(Manifest {
            database: Mutex::new(mobile_world),
            version: response.version.to_owned()
        })
    }
}