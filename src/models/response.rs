use {
    crate::models::types::*,
    serde::{
        Deserialize,
    },
    std::collections::HashMap,
};

#[serde(rename_all = "PascalCase")]
#[derive(Deserialize)]
pub struct GeneralAPIResponse<T>
{
    pub response: T,
    pub error_code: Int32,
    pub throttle_seconds: Int32,
    pub error_status: String,
    pub message: String,
    pub message_data: HashMap<String, String>,
    pub detailed_error_trace: Option<String>,
}