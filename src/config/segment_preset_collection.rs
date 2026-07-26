use serde::{Deserialize, Serialize};
use crate::config::group_type_configuration::GroupTypeConfiguration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentPresetInfo {
    #[serde(default)]
    pub segment_type: serde_json::Value,
    #[serde(default)]
    pub possible_types: Vec<GroupTypeConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentPresetCollection {
    #[serde(default)]
    pub collection_name: String,
    #[serde(default)]
    pub group_type_probabilities: Vec<GroupTypeConfiguration>,
    #[serde(default)]
    pub segment_presets: Vec<SegmentPresetInfo>,
}
