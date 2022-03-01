use {
    crate::models::{
        api::DestinyAPI
    },
    std::env,
    tokio_test::block_on,
};

const STEAM_ID: &str = "76561198261302803";

#[test]
fn get_by_steamid64() {
    let api = DestinyAPI::new(&env::var("BUNGIE_API_KEY").unwrap()).unwrap();
    let usr = block_on(api.get_user_by_steamid64(&STEAM_ID)).unwrap();
    assert_eq!(usr.bungie_net_user.membership_id, 19377351);
}