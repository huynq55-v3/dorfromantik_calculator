use serde::{Deserialize, Serialize};
use crate::utils::UnityRandom;
use crate::config::TileGenFilter;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyframe {
    #[serde(default)]
    pub time: serde_json::Value,
    #[serde(default)]
    pub value: serde_json::Value,
    #[serde(default)]
    pub in_slope: serde_json::Value,
    #[serde(default)]
    pub out_slope: serde_json::Value,
}

impl Keyframe {
    pub fn get_time(&self) -> f32 {
        match &self.time {
            serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0) as f32,
            serde_json::Value::String(s) => {
                if s.eq_ignore_ascii_case("inf") || s.eq_ignore_ascii_case("infinity") {
                    f32::INFINITY
                } else {
                    s.parse::<f32>().unwrap_or(0.0)
                }
            }
            _ => 0.0,
        }
    }

    pub fn get_val(&self) -> f32 {
        match &self.value {
            serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0) as f32,
            serde_json::Value::String(s) => {
                if s.eq_ignore_ascii_case("inf") || s.eq_ignore_ascii_case("infinity") {
                    f32::INFINITY
                } else {
                    s.parse::<f32>().unwrap_or(0.0)
                }
            }
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnimationCurve {
    #[serde(rename = "m_Curve", default)]
    pub curve: Vec<Keyframe>,
}

impl AnimationCurve {
    pub fn evaluate(&self, time: f32) -> f32 {
        if self.curve.is_empty() {
            return 0.0;
        }
        let first_t = self.curve[0].get_time();
        let first_v = self.curve[0].get_val();
        if time <= first_t {
            return first_v;
        }
        let last_idx = self.curve.len() - 1;
        let last_t = self.curve[last_idx].get_time();
        let last_v = self.curve[last_idx].get_val();
        if time >= last_t {
            return last_v;
        }

        for i in 0..self.curve.len() - 1 {
            let t0 = self.curve[i].get_time();
            let t1 = self.curve[i + 1].get_time();
            if time >= t0 && time <= t1 {
                let v0 = self.curve[i].get_val();
                let v1 = self.curve[i + 1].get_val();
                let dt = t1 - t0;
                if dt <= 0.00001 {
                    return v0;
                }
                let alpha = (time - t0) / dt;
                return v0 + alpha * (v1 - v0);
            }
        }
        first_v
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityGroupTypeRef {
    #[serde(rename = "m_FileID", default)]
    pub m_file_id: i32,
    #[serde(rename = "m_PathID", default)]
    pub m_path_id: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestProbabilityEntry {
    #[serde(rename = "quest", default)]
    pub quest: UnityGroupTypeRef,
    #[serde(rename = "probabilityCurve", default)]
    pub probability_curve: AnimationCurve,
    #[serde(rename = "_displayProbability", default)]
    pub display_probability: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestTileOption {
    #[serde(default)]
    pub probability: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestTileSubCollection {
    #[serde(rename = "groupType", default)]
    pub group_type: UnityGroupTypeRef,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "occupiedEdges", default)]
    pub occupied_edges: usize,
    #[serde(rename = "subCollectionRawProbability", default)]
    pub sub_collection_raw_probability: f32,
    #[serde(rename = "subCollectionProbability", default)]
    pub sub_collection_probability: f32,
    #[serde(rename = "_displayProbability", default)]
    pub display_probability: f32,
    #[serde(rename = "questTiles", default)]
    pub quest_tiles: Vec<QuestTileOption>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestTileCollection {
    #[serde(rename = "groupType", default)]
    pub group_type: UnityGroupTypeRef,
    #[serde(rename = "rawProbability", default)]
    pub raw_probability: f32,
    #[serde(rename = "subCollections", default)]
    pub sub_collections: Vec<QuestTileSubCollection>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestSystemConfiguration {
    #[serde(rename = "displayLevel", default)]
    pub display_level: f32,

    #[serde(rename = "questTileProbabilityCurve", default)]
    pub quest_tile_probability_curve: AnimationCurve,

    #[serde(rename = "globalQuestProbabilityMultiplier", default)]
    pub global_quest_probability_multiplier: f32,

    #[serde(rename = "questProbabilities", default)]
    pub quest_probabilities: Vec<QuestProbabilityEntry>,

    #[serde(rename = "questTileCollections", default)]
    pub quest_tile_collections: Vec<QuestTileCollection>,
}

impl QuestSystemConfiguration {
    pub fn load_from_asset_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let sanitized = json_str
            .replace(": Infinity,", ": \"Infinity\",")
            .replace(": -Infinity,", ": \"-Infinity\",");
        let root: serde_json::Value = serde_json::from_str(&sanitized)?;
        let structure = root.get("m_Structure").unwrap_or(&root);
        let mut config: Self = serde_json::from_value(structure.clone())?;

        // TÍNH TOÁN ĐỘNG THỰC TẾ DỰA TRÊN TRỌNG SỐ RAW TỰ JSON VÀ HỆ SỐ HÌNH HỌC (1.4)^edges
        config.update_values_dynamic();

        Ok(config)
    }

    /// Mô phỏng 100% thuật toán C# UpdateValues() với hệ số hình học (1.4)^occupiedEdges
    pub fn update_values_dynamic(&mut self) {
        for col in &mut self.quest_tile_collections {
            for sub in &mut col.sub_collections {
                let edges = sub.occupied_edges as i32;
                let base_raw = sub.sub_collection_raw_probability;
                if edges > 0 && base_raw > 0.0 {
                    // Trọng số RAM = subCollectionRawProbability đọc trực tiếp từ JSON * (1.4)^edges
                    let scale = 1.4f32.powi(edges);
                    sub.sub_collection_raw_probability = base_raw * scale;
                }
            }
        }
    }

    pub fn quest_tile_probability(&self, active_quest_count: usize, _total_tiles_placed: usize) -> f32 {
        let base = self
            .quest_tile_probability_curve
            .evaluate(active_quest_count as f32);
        base * self.global_quest_probability_multiplier
    }

    pub fn apply_session_settings(
        &mut self,
        active_village_prob: f32,
        active_forest_prob: f32,
        active_agri_prob: f32,
        active_water_prob: f32,
        active_train_prob: f32,
        _active_density: f32,
        multiplier: f32,
        _max_active: f32,
        _min_prob: f32,
    ) {
        self.global_quest_probability_multiplier = multiplier;

        for col in &mut self.quest_tile_collections {
            match col.group_type.m_path_id {
                41478 => col.raw_probability = active_forest_prob,
                41479 => col.raw_probability = active_agri_prob,
                41480 => col.raw_probability = active_train_prob,
                41481 => col.raw_probability = active_village_prob,
                41482 => col.raw_probability = active_water_prob,
                _ => {}
            }
        }
    }

    pub fn get_random_quest_tile_filtered(
        &self,
        rng: &mut UnityRandom,
        filter: TileGenFilter,
    ) -> (String, String) {
        let valid_cols: Vec<&QuestTileCollection> = self
            .quest_tile_collections
            .iter()
            .filter(|c| c.raw_probability > 0.0)
            .collect();

        if valid_cols.is_empty() {
            return ("Unknown".to_string(), "QuestTile_Generic".to_string());
        }

        let total_col_weight: f32 = valid_cols.iter().map(|c| c.raw_probability).sum();
        let col_roll = rng.range_float(0.0, total_col_weight);

        let mut roll = col_roll;
        let mut chosen_col = valid_cols[0];
        for c in &valid_cols {
            if roll <= c.raw_probability {
                chosen_col = c;
                break;
            }
            roll -= c.raw_probability;
        }

        let valid_subs: Vec<&QuestTileSubCollection> = chosen_col
            .sub_collections
            .iter()
            .filter(|s| {
                if s.sub_collection_raw_probability <= 0.0 {
                    return false;
                }
                if filter == TileGenFilter::AtLeastTwoEmptyEdges && s.occupied_edges >= 5 {
                    return false;
                }
                true
            })
            .collect();

        if valid_subs.is_empty() {
            return ("Unknown".to_string(), "QuestTile_Generic".to_string());
        }

        let total_sub_weight: f32 = valid_subs.iter().map(|s| s.sub_collection_raw_probability).sum();
        let sub_roll = rng.range_float(0.0, total_sub_weight);

        let mut roll2 = sub_roll;
        let mut chosen_sub = valid_subs[0];
        for s in &valid_subs {
            if roll2 <= s.sub_collection_raw_probability {
                chosen_sub = s;
                break;
            }
            roll2 -= s.sub_collection_raw_probability;
        }

        let valid_opts: Vec<&QuestTileOption> = chosen_sub
            .quest_tiles
            .iter()
            .filter(|o| o.probability > 0.0)
            .collect();

        if !valid_opts.is_empty() {
            let total_opt_weight: f32 = valid_opts.iter().map(|o| o.probability).sum();
            let _opt_roll = rng.range_float(0.0, total_opt_weight);
        }

        (
            chosen_sub.group_type.m_path_id.to_string(),
            chosen_sub.name.clone(),
        )
    }
}
