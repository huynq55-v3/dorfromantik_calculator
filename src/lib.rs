pub mod config;
pub mod core;
pub mod utils;

pub use config::{GroupTypeConfiguration, TileGenConfiguration};
pub use core::GroupType;
pub use utils::{extract_session_data_from_save, GameConfigFile, SessionSaveData};

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub seed: i32,
    pub config_string: String,
    pub tile_gen: TileGenConfiguration,
    pub raw_save_data: SessionSaveData,
    pub raw_config_file: GameConfigFile,
}

impl GameConfig {
    /// Centralized loader pattern: Loads session configuration from .sav and .cfg
    pub fn load(save_path: &str, config_path: &str) -> Result<Self, String> {
        // 1. Decode save file
        let save_data = extract_session_data_from_save(save_path)?;

        // 2. Load TOML rules file
        let config_file = GameConfigFile::load_from_file(config_path)?;

        // 3. Match level indices (rule_type_id: 1..5)
        let village_level = save_data.get_level_index(1, 1, None);
        let forest_level = save_data.get_level_index(2, 2, None);
        let agri_level = save_data.get_level_index(3, 3, None);
        let water_level = save_data.get_level_index(4, 4, None);
        let train_level = save_data.get_level_index(5, 5, None);

        // 4. Look up probability values
        let village_prob = config_file.tile_gen.get_value(&config_file.tile_gen.village_prob, village_level);
        let forest_prob = config_file.tile_gen.get_value(&config_file.tile_gen.forest_prob, forest_level);
        let agri_prob = config_file.tile_gen.get_value(&config_file.tile_gen.agriculture_prob, agri_level);
        let water_prob = config_file.tile_gen.get_value(&config_file.tile_gen.water_prob, water_level);
        let train_prob = config_file.tile_gen.get_value(&config_file.tile_gen.train_track_prob, train_level);

        // 5. Construct TileGenConfiguration with globalGroupTypeProbabilities
        let global_probabilities = vec![
            GroupTypeConfiguration::new(GroupType::Village, village_prob),
            GroupTypeConfiguration::new(GroupType::Forest, forest_prob),
            GroupTypeConfiguration::new(GroupType::Agriculture, agri_prob),
            GroupTypeConfiguration::new(GroupType::Water, water_prob),
            GroupTypeConfiguration::new(GroupType::TrainTrack, train_prob),
        ];

        let tile_gen = TileGenConfiguration::new(global_probabilities);

        Ok(Self {
            seed: save_data.real_tile_seed,
            config_string: save_data.config_string.clone(),
            tile_gen,
            raw_save_data: save_data,
            raw_config_file: config_file,
        })
    }
}
