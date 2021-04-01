use {
    crate::models::{
        types::*,
        membership::MembershipType,
        api::DestinyAPI,
        profile::{
            Component,
            DestinyProfileResponse
        },
        manifest::{
            Manifest,
            ManifestKey
        },
        manifest_models,
        components::character::CharacterComponent,
    },
    anyhow::{
        Result,
        anyhow,
    },
    async_trait::async_trait,
};

pub trait BNGMembershipID: Sync {
    /// returns a Bungie.net membershipID from a struct
    fn bng_membership_id(&self) -> Int64;
}

impl BNGMembershipID for Int64 {
    fn bng_membership_id(&self) -> Int64 {
        *self
    }
}

#[async_trait]
pub trait DestinyMembershipID: Sync {
    /// returns a Destiny membershipID from  a struct
    fn destiny_membership_id(&self) -> Int64;

    async fn get_components<P: PlatformType>(&self, api: &DestinyAPI, platform: &P, components: Vec<Component>) -> Result<DestinyProfileResponse> {
        Ok(api.get_components(&self.destiny_membership_id(), platform, components).await?)
    }

    async fn get_characters<P: PlatformType>(&self, api: &DestinyAPI, platform: &P) -> Result<Vec<CharacterComponent>> {
        let response = api.get_components(&self.destiny_membership_id(), platform, vec![Component::Characters]).await?;
        match response.characters {
            Some(x) => {
                let chars = match x.data {
                    Some(x) => {
                        x.values().cloned().collect()
                    },
                    None => return Err(anyhow!("data was None!"))
                };
                return Ok(chars)
            },
            None => return Err(anyhow!("characters was None!"))
        }
    }

    async fn get_seasons<P: PlatformType>(&self, api: &DestinyAPI, manifest: &Manifest, platform: &P) -> Result<Vec<manifest_models::Season>> {
        let response = api.get_components(&self.destiny_membership_id(), platform, vec![Component::Profiles]).await?;

        match response.profile {
            Some(x) => {
                let season_hashes = match x.data {
                    Some(x) => {
                        x.season_hashes
                    },
                    None => return Err(anyhow!("data was None!"))
                };

                let mut result = Vec::new();
                for season_hash in season_hashes {
                    result.push(manifest.query(season_hash, ManifestKey::Season)?);
                }
                return Ok(result)
            },
            None => return Err(anyhow!("profile was None!"))
        }
    }
}

impl DestinyMembershipID for Int64 {
    fn destiny_membership_id(&self) -> Int64 {
        *self
    }
}

pub trait PlatformType: Sync {
    fn platform_type(&self) -> MembershipType;
}

impl PlatformType for MembershipType {
    fn platform_type(&self) -> MembershipType {
        *self
    }
}

#[async_trait]
pub trait CharacterID {
    fn character_id(&self) -> Int64;
}