use std::io::Write;
use std::path::PathBuf;
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

    if let Some(arg) = std::env::args().skip(1).next() {
        if &arg == "--ci-output-manifest-url" {
            let path = std::env::var("CI_OUTPUT_MANIFEST_URL_FILE").expect("CI_OUTPUT_MANIFEST_URL_FILE not set");
            output_manifest_url_to_file(&api, PathBuf::from(path)).await;
            return;
        } else {
            log::warn!("Invalid argument: {}", arg);
        }
    }

    let manifest_path = get_recommended_manifest_path().expect("Could not find recommended manifest path");

    info!("Using manifest path: {}", manifest_path.display());
    std::fs::create_dir_all(&manifest_path).expect("Could not create manifest directory");

    info!("Getting manifest...");
    api.manifest(manifest_path, Locale::English).await.expect("Failed to download manifest");
    info!("Done!");
}

async fn output_manifest_url_to_file(api: &DestinyAPI, path: PathBuf) {
    let resp = api.call_get_manifest().await.unwrap();
    let url = resp.response.mobile_world_content_paths.get(&format!("{}", Locale::English)).unwrap();

    info!("Creating file: {}", path.display());
    let mut file = std::fs::File::create(&path).unwrap();

    file.write_all(url.as_bytes()).unwrap();
    info!("wrote manifest URL to file: {}", path.display());
}