use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json;
use reqwest;
use anyhow;

//pub type Byte = u8;
//pub type Int16 = i16;
//pub type Int32 = i32;
//pub type Int64 = i64;
//pub type Uint16 = i16;
//pub type Uint32 = u32;
//pub type Uint64 = u64;
//pub type Float32 = f32;
//pub type Float64 = f64;

pub type MessageData = std::collections::HashMap<String, String>;

async fn get_request<T: DeserializeOwned>(client: &reqwest::Client, url: &str) -> anyhow::Result<T> {
    Ok(serde_json::from_str(&client.get(format!("{}{}", BASE_API_PATH, url)).send().await?.text().await?)?)
}

