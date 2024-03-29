Destiny-rs
==========

[![license](https://img.shields.io/crates/l/destiny-rs)](LICENSE)
[![docs](https://docs.rs/destiny_rs/badge.svg)](https://docs.rs/destiny_rs)
[![Rust build/unit tests](https://github.com/zeskeertwee/destiny-rs/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/zeskeertwee/destiny-rs/actions/workflows/test.yml)

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
    let api = DestinyAPI::new(&env::var("API_KEY").unwrap()).unwrap();

    let manifest_path = destiny_rs::get_recommended_manifest_path().unwrap();

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

Contributing
=========
This library is currently not complete, and doesn't cover all structs and endpoints of the bungie.net API. The goal is to eventually support all of the endpoints.

To contribute, you should first open an issue (or assign a existing open issue to yourself) so that everyone can see you're working on something.
Then, when you want to merge your code, you can open a PR to this repository.
