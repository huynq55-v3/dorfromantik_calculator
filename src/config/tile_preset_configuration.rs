use serde::{Deserialize, Serialize};
use crate::config::segment_preset_collection::SegmentPresetInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilePresetConfiguration {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tile_preset: serde_json::Value,
    #[serde(default)]
    pub raw_probability: f32,
    #[serde(default)]
    pub tile_preset_probability: f32,
    #[serde(default, rename = "_displayProbability")]
    pub display_probability: f32,
    #[serde(default)]
    pub segment_probabilities: Vec<SegmentPresetInfo>,
    #[serde(default)]
    pub occupied_edges: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilePresetConfigurationSubCollection {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub probability: f32,
    #[serde(default)]
    pub sub_collection_probability: f32,
    #[serde(default)]
    pub sub_collection_raw_probability: f32,
    #[serde(default, rename = "_displayProbability")]
    pub display_probability: f32,
    #[serde(default)]
    pub tile_presets: Vec<TilePresetConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilePresetConfigurationCollection {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub probability: f32,
    #[serde(default)]
    pub collection_probability: f32,
    #[serde(default)]
    pub collection_raw_probability: f32,
    #[serde(default, rename = "_displayProbability")]
    pub display_probability: f32,
    #[serde(default)]
    pub sub_collections: Option<Vec<TilePresetConfigurationSubCollection>>,
    #[serde(default)]
    pub tile_presets: Option<Vec<TilePresetConfiguration>>,
}
