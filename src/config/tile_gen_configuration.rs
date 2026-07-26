use serde::{Deserialize, Serialize};
use crate::config::group_type_configuration::GroupTypeConfiguration;
use crate::config::segment_preset_collection::{SegmentPresetCollection, SegmentPresetInfo};
use crate::config::tile_preset_configuration::{
    TilePresetConfiguration, TilePresetConfigurationCollection,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileGenFilter {
    AtLeastTwoEmptyEdges,
    None,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileGenConfigurationData {
    #[serde(default = "default_prob")]
    pub default_probability: f32,
    #[serde(default = "default_true")]
    pub auto_update: bool,
    #[serde(default)]
    pub global_group_type_probabilities: Vec<GroupTypeConfiguration>,
    #[serde(default)]
    pub segment_preset_collections: Vec<SegmentPresetCollection>,
    #[serde(default)]
    pub tile_preset_collections: Vec<TilePresetConfigurationCollection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileGenConfigurationAsset {
    #[serde(rename = "m_Name", default)]
    pub name: String,
    #[serde(rename = "m_Structure")]
    pub structure: TileGenConfigurationData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileGenConfiguration {
    pub default_probability: f32,
    pub auto_update: bool,
    pub global_group_type_probabilities: Vec<GroupTypeConfiguration>,
    pub segment_preset_collections: Vec<SegmentPresetCollection>,
    pub tile_preset_collections: Vec<TilePresetConfigurationCollection>,
    #[serde(skip)]
    pub all_tile_presets: Vec<TilePresetConfiguration>,
    #[serde(skip)]
    pub all_segment_presets: Vec<SegmentPresetInfo>,
}

fn default_prob() -> f32 {
    10.0
}

fn default_true() -> bool {
    true
}

impl TileGenConfiguration {
    pub fn new(global_group_type_probabilities: Vec<GroupTypeConfiguration>) -> Self {
        let mut config = Self {
            default_probability: 10.0,
            auto_update: true,
            global_group_type_probabilities,
            segment_preset_collections: Vec::new(),
            tile_preset_collections: Vec::new(),
            all_tile_presets: Vec::new(),
            all_segment_presets: Vec::new(),
        };
        config.update_values();
        config
    }

    /// Load trực tiếp từ file JSON Asset dump từ Unity Engine
    pub fn load_from_asset_json(json_str: &str) -> Result<Self, String> {
        let asset: TileGenConfigurationAsset = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse TileGenConfiguration JSON asset: {}", e))?;

        let data = asset.structure;
        let mut config = Self {
            default_probability: data.default_probability,
            auto_update: data.auto_update,
            global_group_type_probabilities: data.global_group_type_probabilities,
            segment_preset_collections: data.segment_preset_collections,
            tile_preset_collections: data.tile_preset_collections,
            all_tile_presets: Vec::new(),
            all_segment_presets: Vec::new(),
        };

        config.update_values();
        Ok(config)
    }

    /// Tương ứng với void UpdateValues() trong C# Unity
    pub fn update_values(&mut self) {
        self.update_global_group_type_probabilities();
        self.update_all_tile_presets_list();
        self.update_all_segments_list();
    }

    /// Tương ứng với void UpdateGlobalGroupTypeProbabilities() trong C#
    fn update_global_group_type_probabilities(&mut self) {
        let total: f32 = self
            .global_group_type_probabilities
            .iter()
            .map(|x| x.raw_probability)
            .sum();

        for item in &mut self.global_group_type_probabilities {
            item.probability_in_percent = if total == 0.0 {
                0.0
            } else {
                item.raw_probability / total
            };
            item.display_probability = item.probability_in_percent * 100.0;
        }
    }

    /// Tương ứng với void UpdateAllTilePresetsList() trong C#
    pub fn update_all_tile_presets_list(&mut self) {
        self.all_tile_presets.clear();
        for col in &self.tile_preset_collections {
            if let Some(sub_cols) = &col.sub_collections {
                for sub in sub_cols {
                    self.all_tile_presets.extend(sub.tile_presets.clone());
                }
            }
            if let Some(presets) = &col.tile_presets {
                self.all_tile_presets.extend(presets.clone());
            }
        }
    }

    /// Tương ứng với void UpdateAllSegmentsList() trong C#
    pub fn update_all_segments_list(&mut self) {
        self.all_segment_presets.clear();
        for col in &self.segment_preset_collections {
            self.all_segment_presets.extend(col.segment_presets.clone());
        }
    }

    /// Tương ứng với List<TilePresetConfiguration> GetFilteredTilePresets(TileGenFilter usedFilter) trong C#
    pub fn get_filtered_tile_presets(&self, used_filter: TileGenFilter) -> Vec<&TilePresetConfiguration> {
        match used_filter {
            TileGenFilter::AtLeastTwoEmptyEdges => {
                self.all_tile_presets.iter().filter(|x| x.occupied_edges < 5).collect()
            }
            TileGenFilter::None => self.all_tile_presets.iter().collect(),
        }
    }
}
