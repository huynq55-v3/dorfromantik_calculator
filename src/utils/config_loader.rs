use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct TileGenConfigTable {
    pub village_prob: Vec<f32>,
    pub forest_prob: Vec<f32>,
    pub agriculture_prob: Vec<f32>,
    pub water_prob: Vec<f32>,
    pub train_track_prob: Vec<f32>,
    pub density: Vec<f32>,
}

impl TileGenConfigTable {
    pub fn get_value(&self, table: &[f32], level: usize) -> f32 {
        if level < table.len() {
            table[level]
        } else {
            0.0
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct QuestSystemConfigTable {
    pub quest_prob: Vec<f32>,
    pub quest_difficulty: Vec<f32>,
    pub flag_quest_prob: Vec<f32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorldConfigTable {
    pub tile_stack_height: Vec<toml::Value>,
    pub tile_limit: Vec<f32>,
    pub world_border_radius: Vec<f32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameConfigFile {
    pub tile_gen: TileGenConfigTable,
    pub quest_system: QuestSystemConfigTable,
    pub world: WorldConfigTable,
}

impl GameConfigFile {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file {}: {}", path, e))?;
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse TOML config {}: {}", path, e))
    }
}
