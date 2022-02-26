use {crate::models::types::*, serde::Deserialize};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Ignores-IgnoreResponse.html#schema_Ignores-IgnoreResponse)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IgnoreResponse {
    pub is_ignored: bool,
    pub ignore_flags: Int32,
}
