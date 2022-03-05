use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json;
use reqwest;

//pub type Byte = u8;
//pub type Int16 = i16;
//pub type Int32 = i32;
//pub type Int64 = i64;
//pub type Uint16 = i16;
//pub type Uint32 = u32;
//pub type Uint64 = u64;
//pub type Float32 = f32;
//pub type Float64 = f64;

async fn get_request<T: DeserializeOwned>(client: &reqwest::Client, url: &str) -> Result<T, reqwest::Error> {
    Ok(serde_json::from_str(&client.get(format!("{}{}", BASE_API_PATH, url)).send().await?.text().await?))
}

mod app {
    pub fn get_application_api_usage(client: &reqwest::Client, application_id: i32) -> Result<crate::applications::ApiUsage, reqwest::Error> {
        get_request(client, &format!("/App/ApiUsage/{}", application_id)).await
    }
}
