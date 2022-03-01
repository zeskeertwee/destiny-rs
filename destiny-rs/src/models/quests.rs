use {
    crate::models::types::*,
    serde::Deserialize
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveProgress {
    /// mapped to [`Objective`](crate::models::manifest::ManifestKey::Objective)
    // TODO
    pub objective_hash: Uint32,
    /// mapped to [`Destination`](crate::models::manifest::ManifestKey::Destination)
    // TODO
    pub destination_hash: Option<Uint32>,
    /// mapped to [`Activity`](crate::models::manifest::ManifestKey::Activity)
    // TODO
    pub activity_hash: Option<Uint32>,
    pub progress: Int32,
    pub completion_value: Int32,
    pub complete: bool,
    pub visible: bool,
}