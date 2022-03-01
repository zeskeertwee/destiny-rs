use {
    crate::models::types::*,
    serde::Deserialize
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveProgress {
    /// mapped to [`Objective`](crate::models::manifest::ManifestKey::Objective)
    pub objective_hash: Hash,
    /// mapped to [`Destination`](crate::models::manifest::ManifestKey::Destination)
    pub destination_hash: Option<Hash>,
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    pub activity_hash: Option<Hash>,
    pub progress: Int32,
    pub completion_value: Int32,
    pub complete: bool,
    pub visible: bool,
}