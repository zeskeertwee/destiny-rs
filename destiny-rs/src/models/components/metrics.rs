use {
    crate::models::{
        types::*,
        quests::ObjectiveProgress,
    },
    serde::Deserialize,
    std::collections::HashMap,
};
use crate::models::manifest_models::PresentationNode;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsComponent {
    #[serde(deserialize_with = "uint32_map_from_str")]
    pub metrics: HashMap<Uint32, MetricSubComponent>,
    /// mapped to [`PresentationNode`](crate::models::manifest::ManifestKey::PresentationNode)
    pub metrics_root_node_hash: Hash<PresentationNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricSubComponent {
    pub invisible: bool,
    pub objective_progress: ObjectiveProgress,
}