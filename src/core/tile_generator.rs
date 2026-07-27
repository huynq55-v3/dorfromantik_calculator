use crate::config::{QuestSystemConfiguration, TileGenConfiguration, TileGenFilter};
use crate::core::quest_manager::QuestManager;
use crate::utils::UnityRandom;

#[derive(Debug, Clone)]
pub struct GeneratedTileInfo {
    pub tile_index: usize,
    pub tile_seed: i32,
    pub is_quest: bool,
    pub preset_name: String,
    pub occupied_edges: usize,
    pub rotation: usize,
}

#[derive(Debug, Clone)]
pub struct TileGenerator {
    pub tile_generation_seed: i32,
    pub generated_tile_count: usize,
    pub generated_quest_count: usize,
    pub tile_seed_increment_step: i32,
    pub at_least_two_empty_edges_for_x_turns: usize,
    pub call_counter: usize,
}

impl TileGenerator {
    pub fn new(seed: i32) -> Self {
        let mut rng = UnityRandom::new(seed);
        let step_float = rng.value();
        let step = -100000 + (step_float * 200000.0) as i32;
        Self {
            tile_generation_seed: seed,
            generated_tile_count: 0,
            generated_quest_count: 0,
            tile_seed_increment_step: step,
            at_least_two_empty_edges_for_x_turns: 5,
            call_counter: 0,
        }
    }

    pub fn set_step(&mut self, step: i32) {
        self.tile_seed_increment_step = step;
    }

    pub fn generate_tile(
        &mut self,
        tile_gen_config: &TileGenConfiguration,
        quest_config: &QuestSystemConfiguration,
        quest_manager: &mut QuestManager,
        overwrite_quest_prob: Option<f32>,
    ) -> GeneratedTileInfo {
        // Tương đương Dorfromantik2.cs:L43168:
        // int num = TileGenerationSeed + (generatedTileCount - GeneratedQuestCount) * step;
        let base_seed = self.tile_generation_seed.wrapping_add(
            ((self.generated_tile_count - self.generated_quest_count) as i32)
                .wrapping_mul(self.tile_seed_increment_step),
        );

        self.generated_tile_count += 1;

        // Tương đương Dorfromantik2.cs:L43171:
        let filter = if self.generated_tile_count <= self.at_least_two_empty_edges_for_x_turns {
            TileGenFilter::AtLeastTwoEmptyEdges
        } else {
            TileGenFilter::None
        };

        // Tương đương Dorfromantik2.cs:L43172-L43173:
        // Random.InitState(TileGenerationSeed + generatedTileCount * step); // KHÔNG nhân 2!
        // float value = Random.value;
        let roll_seed = self.tile_generation_seed.wrapping_add(
            (self.generated_tile_count as i32).wrapping_mul(self.tile_seed_increment_step),
        );
        let mut prob_rng = UnityRandom::new(roll_seed);
        let rand_val: f32 = prob_rng.value(); // Gọi 1 LẦN duy nhất theo đúng C#

        let is_quest = match overwrite_quest_prob {
            Some(p) if p >= 0.0 => rand_val <= p,
            _ => quest_manager.should_generate_quest(quest_config, self.generated_tile_count, rand_val),
        };

        if is_quest {
            // Tương đương Dorfromantik2.cs:L43176-L43179:
            // int num2 = TileGenerationSeed + GeneratedQuestCount * step;
            let quest_seed = self.tile_generation_seed.wrapping_add(
                (self.generated_quest_count as i32).wrapping_mul(self.tile_seed_increment_step),
            );
            self.generated_quest_count += 1;
            quest_manager.add_quest();

            let quest_init_seed = quest_seed.wrapping_mul(2);
            let mut quest_rng = UnityRandom::new(quest_init_seed);
            let (_cat, quest_prefab_name) = quest_config.get_random_quest_tile_filtered(&mut quest_rng, filter);

            return GeneratedTileInfo {
                tile_index: self.generated_tile_count,
                tile_seed: quest_init_seed,
                is_quest: true,
                preset_name: quest_prefab_name,
                occupied_edges: 2,
                rotation: 0,
            };
        }

        // Tương đương Dorfromantik2.cs:L43187-L43189: Sinh bài thường (Non-Quest Tile)
        let mut tile_rng = UnityRandom::new(base_seed.wrapping_mul(2));
        let filtered_presets = tile_gen_config.get_filtered_tile_presets(filter);

        let selected_preset = if !filtered_presets.is_empty() {
            let total_weight: f32 = filtered_presets.iter().map(|p| p.raw_probability).sum();
            let roll_weighted = tile_rng.range_float(0.0, total_weight);

            let mut roll = roll_weighted;
            let mut chosen = filtered_presets[0];
            for p in &filtered_presets {
                if roll <= p.raw_probability {
                    chosen = p;
                    break;
                }
                roll -= p.raw_probability;
            }
            chosen
        } else {
            &tile_gen_config.all_tile_presets[0]
        };

        let rot: i32 = tile_rng.range_int(0, 6);

        GeneratedTileInfo {
            tile_index: self.generated_tile_count,
            tile_seed: base_seed,
            is_quest: false,
            preset_name: selected_preset.name.clone(),
            occupied_edges: selected_preset.occupied_edges,
            rotation: rot as usize,
        }
    }
}
