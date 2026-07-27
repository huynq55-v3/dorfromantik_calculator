use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

    pub fn apply_session_settings(
        &mut self,
        village_prob: f32,
        forest_prob: f32,
        agri_prob: f32,
        water_prob: f32,
        train_prob: f32,
        _density: f32,
    ) {
        for group in &mut self.global_group_type_probabilities {
            let name_lower = group.name.to_lowercase();
            if name_lower.contains("village") {
                group.raw_probability = village_prob;
            } else if name_lower.contains("forest") {
                group.raw_probability = forest_prob;
            } else if name_lower.contains("agri") {
                group.raw_probability = agri_prob;
            } else if name_lower.contains("water") {
                group.raw_probability = water_prob;
            } else if name_lower.contains("train") {
                group.raw_probability = train_prob;
            }
        }

        self.update_values();
    }

    pub fn update_values(&mut self) {
        self.update_global_group_type_probabilities();
        self.update_all_tile_presets_list();
        self.update_all_segments_list();
    }

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

    pub fn update_all_tile_presets_list(&mut self) {
        let mut temp = Vec::new();
        for col in &mut self.tile_preset_collections {
            if let Some(sub_cols) = &mut col.sub_collections {
                for sub in sub_cols {
                    let sum_raw: f32 = sub.tile_presets.iter().map(|p| p.raw_probability).sum();
                    for p in &mut sub.tile_presets {
                        p.tile_preset_probability = if sum_raw == 0.0 {
                            0.0
                        } else {
                            (p.raw_probability / sum_raw) * sub.sub_collection_probability
                        };
                    }
                    temp.extend(sub.tile_presets.clone());
                }
            }
            if let Some(presets) = &mut col.tile_presets {
                let sum_raw: f32 = presets.iter().map(|p| p.raw_probability).sum();
                for p in presets.iter_mut() {
                    p.tile_preset_probability = if sum_raw == 0.0 {
                        0.0
                    } else {
                        (p.raw_probability / sum_raw) * col.collection_probability
                    };
                }
                temp.extend(presets.clone());
            }
        }

        // Thứ tự mảng Memory gốc Unity (71 Preset)
        let order_map: HashMap<&str, usize> = [
            ("0A", 0), ("1A", 1), ("1A_1A", 2), ("1A_1A_1A", 3), ("1A_1A_1A_1A", 4),
            ("1A_1A_1A_1A_1A", 5), ("1A_1A_1A_1A_1A_1A", 6), ("2A", 7), ("2A_1A", 8),
            ("2A_1A_1A", 9), ("2A_1A_1A_1A", 10), ("2A_1A_1A_1A_1A", 11), ("2A_2A", 12),
            ("2A_2A_1A", 13), ("2A_2A_1A_1A", 14), ("2A_2A_2A", 15), ("2B", 16),
            ("2B_1A", 17), ("2B_1A_1A", 18), ("2B_1A_1A_1A", 19), ("2B_1A_1A_1A_1A", 20),
            ("2B_2A", 21), ("2B_2A_1A", 22), ("2B_2A_1A_1A", 23), ("2C", 24),
            ("2C_1A", 25), ("2C_1A_1A", 26), ("2C_1A_1A_1A", 27), ("2C_1A_1A_1A_1A", 28),
            ("2C_2A", 29), ("2C_2A_1A", 30), ("2C_2A_1A_1A", 31), ("2C_2A_2A", 32),
            ("3A", 33), ("3A_1A", 34), ("3A_1A_1A", 35), ("3A_1A_1A_1A", 36),
            ("3A_2A", 37), ("3A_2A_1A", 38), ("3A_2B", 39), ("3A_2B_1A", 40),
            ("3A_3A", 41), ("3B", 42), ("3B_1A", 43), ("3B_1A_1A", 44),
            ("3B_1A_1A_1A", 45), ("3B_2A", 46), ("3B_2A_1A", 47), ("3C", 48),
            ("3C_1A", 49), ("3C_1A_1A", 50), ("3C_1A_1A_1A", 51), ("3C_2A", 52),
            ("3C_2A_1A", 53), ("3D", 54), ("3D_1A", 55), ("3D_1A_1A", 56),
            ("3D_1A_1A_1A", 57), ("4A", 58), ("4A_1A", 59), ("4A_1A_1A", 60),
            ("4A_2A", 61), ("4B", 62), ("4B_1A", 63), ("4B_1A_1A", 64),
            ("4C", 65), ("4C_1A", 66), ("4C_1A_1A", 67), ("5A", 68),
            ("5A_1A", 69), ("6A", 70)
        ].iter().cloned().collect();

        temp.sort_by_key(|p| *order_map.get(p.name.as_str()).unwrap_or(&999));
        self.all_tile_presets = temp;
    }

    pub fn update_all_segments_list(&mut self) {
        self.all_segment_presets.clear();
        for col in &self.segment_preset_collections {
            self.all_segment_presets.extend(col.segment_presets.clone());
        }
    }

    pub fn get_filtered_tile_presets(&self, used_filter: TileGenFilter) -> Vec<&TilePresetConfiguration> {
        match used_filter {
            TileGenFilter::AtLeastTwoEmptyEdges => {
                self.all_tile_presets.iter().filter(|x| x.occupied_edges < 5).collect()
            }
            TileGenFilter::None => self.all_tile_presets.iter().collect(),
        }
    }
}
