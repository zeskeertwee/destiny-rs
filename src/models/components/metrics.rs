use {
    crate::models::{
        types::*,
        quests::ObjectiveProgress,
    },
    serde::Deserialize,
    std::collections::HashMap,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsComponent {
    #[serde(deserialize_with = "uint32_map_from_str")]
    pub metrics: HashMap<Hash, MetricSubComponent>,
    /// mapped to [`PresentationNode`](crate::models::manifest::ManifestKey::PresentationNode)
    pub metrics_root_node_hash: Hash,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricSubComponent {
    pub invisible: bool,
    pub objective_progress: ObjectiveProgress,
}