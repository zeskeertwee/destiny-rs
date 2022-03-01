use destiny_rs::{
    get_recommended_manifest_path,
    models::api::DestinyAPI
};
use destiny_rs::models::locale::Locale;
use tokio;
use log::info;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();
    let key = std::env::var("BUNGIE_API_KEY").expect("BUNGIE_API_KEY not set");
    let api = DestinyAPI::new(&key).expect("Failed to create DestinyAPI");

    let manifest_path = get_recommended_manifest_path().expect("Could not find recommended manifest path");

    info!("Using manifest path: {}", manifest_path.display());
    std::fs::create_dir_all(&manifest_path).expect("Could not create manifest directory");

    info!("Getting manifest...");
    api.manifest(manifest_path, Locale::English).await.expect("Failed to download manifest");
    info!("Done!");
}