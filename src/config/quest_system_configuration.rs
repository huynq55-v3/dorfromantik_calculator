use serde::{Deserialize, Serialize};

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
        let last_t = self.curve.last().unwrap().get_time();
        let last_v = self.curve.last().unwrap().get_val();
        if time >= last_t {
            return last_v;
        }
        for i in 0..self.curve.len() - 1 {
            let k1 = &self.curve[i];
            let k2 = &self.curve[i + 1];
            let k1_t = k1.get_time();
            let k2_t = k2.get_time();
            let k1_v = k1.get_val();
            let k2_v = k2.get_val();
            if time >= k1_t && time <= k2_t {
                if k2_t == k1_t {
                    return k1_v;
                }
                let t = (time - k1_t) / (k2_t - k1_t);
                return k1_v + t * (k2_v - k1_v);
            }
        }
        0.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestTileOption {
    #[serde(default)]
    pub quest_tile: serde_json::Value,
    #[serde(default)]
    pub probability: f32,
    #[serde(default)]
    pub quest_options: Vec<serde_json::Value>,
    #[serde(default)]
    pub unlock_reward: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestTileSubCollection {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub group_type: serde_json::Value,
    #[serde(default)]
    pub all_segment_types: Vec<serde_json::Value>,
    #[serde(default)]
    pub occupied_edges: usize,
    #[serde(default)]
    pub sub_collection_raw_probability: f32,
    #[serde(default)]
    pub sub_collection_probability: f32,
    #[serde(default, rename = "_displayProbability")]
    pub display_probability: f32,
    #[serde(default)]
    pub quest_tiles: Vec<QuestTileOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestTileCollection {
    #[serde(default)]
    pub group_type: serde_json::Value,
    #[serde(default)]
    pub raw_probability: f32,
    #[serde(default)]
    pub collection_probability: f32,
    #[serde(default, rename = "_displayProbability")]
    pub display_probability: f32,
    #[serde(default)]
    pub default_quest_options: Vec<serde_json::Value>,
    #[serde(default)]
    pub sub_collections: Vec<QuestTileSubCollection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestProbability {
    #[serde(default)]
    pub quest: serde_json::Value,
    #[serde(default)]
    pub probability_curve: AnimationCurve,
    #[serde(default, rename = "_displayProbability")]
    pub display_probability: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestSystemConfigurationData {
    #[serde(default)]
    pub auto_update: bool,
    #[serde(default)]
    pub quest_tile_probability_curve: AnimationCurve,
    #[serde(default)]
    pub display_level: usize,
    #[serde(default)]
    pub quest_probabilities: Vec<QuestProbability>,
    #[serde(default)]
    pub global_lock_quest_probability: serde_json::Value,
    #[serde(default)]
    pub global_quest_probability_multiplier: f32,
    #[serde(default)]
    pub global_flag_quest_probability_multiplier: f32,
    #[serde(default)]
    pub exponential_difficulty_increase_factor: f32,
    #[serde(default)]
    pub quest_spawn_tile_limit: i32,
    #[serde(default)]
    pub quest_tile_collections: Vec<QuestTileCollection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSystemConfigurationAsset {
    #[serde(rename = "m_Name", default)]
    pub name: String,
    #[serde(rename = "m_Structure")]
    pub structure: QuestSystemConfigurationData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestSystemConfiguration {
    pub auto_update: bool,
    pub quest_tile_probability_curve: AnimationCurve,
    pub display_level: usize,
    pub quest_probabilities: Vec<QuestProbability>,
    pub global_quest_probability_multiplier: f32,
    pub global_flag_quest_probability_multiplier: f32,
    pub exponential_difficulty_increase_factor: f32,
    pub quest_spawn_tile_limit: i32,
    pub quest_tile_collections: Vec<QuestTileCollection>,
}

impl QuestSystemConfiguration {
    pub fn load_from_asset_json(json_str: &str) -> Result<Self, String> {
        let sanitized = json_str
            .replace(": Infinity", ": \"Infinity\"")
            .replace(": -Infinity", ": \"-Infinity\"");

        let asset: QuestSystemConfigurationAsset = serde_json::from_str(&sanitized)
            .map_err(|e| format!("Failed to parse QuestSystemConfiguration JSON asset: {}", e))?;

        let data = asset.structure;
        let mut config = Self {
            auto_update: data.auto_update,
            quest_tile_probability_curve: data.quest_tile_probability_curve,
            display_level: data.display_level,
            quest_probabilities: data.quest_probabilities,
            global_quest_probability_multiplier: data.global_quest_probability_multiplier,
            global_flag_quest_probability_multiplier: data.global_flag_quest_probability_multiplier,
            exponential_difficulty_increase_factor: data.exponential_difficulty_increase_factor,
            quest_spawn_tile_limit: data.quest_spawn_tile_limit,
            quest_tile_collections: data.quest_tile_collections,
        };

        config.update_values(false);
        Ok(config)
    }

    pub fn quest_tile_probability(&self, active_quest_count: usize, total_tile_count: usize) -> f32 {
        if self.quest_spawn_tile_limit > 0 && total_tile_count >= self.quest_spawn_tile_limit as usize {
            return 0.0;
        }
        self.quest_tile_probability_curve.evaluate(active_quest_count as f32)
            * self.global_quest_probability_multiplier
    }

    pub fn set_global_multiplier_values(
        &mut self,
        quest_prob_mult: f32,
        _quest_diff_mult: f32,
        flag_quest_prob_mult: f32,
    ) {
        self.global_quest_probability_multiplier = quest_prob_mult;
        self.global_flag_quest_probability_multiplier = flag_quest_prob_mult;
    }

    pub fn update_values(&mut self, _update_segment_types: bool) {
        let num: f32 = self
            .quest_tile_collections
            .iter()
            .map(|x| x.raw_probability)
            .sum();

        for col in &mut self.quest_tile_collections {
            col.collection_probability = if num == 0.0 {
                0.0
            } else {
                col.raw_probability / num
            };
            col.display_probability = col.collection_probability * 500.0;

            let num2: f32 = col
                .sub_collections
                .iter()
                .map(|x| x.sub_collection_raw_probability)
                .sum();

            for sub in &mut col.sub_collections {
                sub.sub_collection_probability = if num2 == 0.0 {
                    0.0
                } else {
                    sub.sub_collection_raw_probability / num2 * col.collection_probability
                };
                sub.display_probability = sub.sub_collection_probability * 500.0;
            }
        }
    }
}
