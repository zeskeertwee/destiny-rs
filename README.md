Destiny-rs
==========

[![license](https://img.shields.io/crates/l/destiny-rs)](LICENSE.md)
[![docs](https://docs.rs/destiny_rs/badge.svg)](https://docs.rs/destiny_rs)

A library for interacting with Bungie's destiny 2 api, and manifest

A simple example that looks up someone's characters with their bungie.net id, and queries the manifest multiple times:
```rust
use destiny_rs::models::{api::DestinyAPI, locale::Locale, membership::MembershipType, manifest::ManifestKey, manifest_models};
use destiny_rs::traits::id::DestinyMembershipID;
use tokio;
use std::env;
use std::path::PathBuf;

#[tokio::main]
pub async fn main() {
    let api = DestinyAPI::new(&env::var("API_KEY").unwrap());

    let mut manifest_path = PathBuf::new();
    manifest_path.push("./manifest");

    let manifest = api.manifest(manifest_path, Locale::English).await.unwrap();
    
    let user = api.get_user_membership_data_by_membershipid_bng(&19377351,  &MembershipType::Steam).await.unwrap();
    let characters = user.get_characters(&api, &MembershipType::Steam).await.unwrap();

    println!("{}'s characters:", user.bungie_net_user.display_name);
    for character in characters {
        // these don't do any api calls, they just query the downloaded manifest
        let race: manifest_models::Race = manifest.query(character.race_hash, ManifestKey::Race).unwrap();
        let gender: manifest_models::Gender = manifest.query(character.gender_hash, ManifestKey::Gender).unwrap();
        let class: manifest_models::Class = manifest.query(character.class_hash, ManifestKey::Class).unwrap();

        println!(
            "{} {} {} is power level {}",
            gender.display_properties.name,
            race.display_properties.name,
            class.display_properties.name,
            character.stats.power,
        );
    }
}
```