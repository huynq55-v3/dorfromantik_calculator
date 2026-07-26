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

    fn log_init_state(&mut self, seed: i32, caller: &str) {
        self.call_counter += 1;
        println!(
            " #{:<7} | Random.InitState       | Seed = {:<23} | By: {}",
            self.call_counter, seed, caller
        );
    }

    fn log_random_value(&mut self, val: f32, caller: &str) {
        self.call_counter += 1;
        println!(
            " #{:<7} | Random.value           | -> {:<26.6} | By: {}",
            self.call_counter, val, caller
        );
    }

    fn log_random_range_float(&mut self, min: f32, max: f32, val: f32, caller: &str) {
        self.call_counter += 1;
        println!(
            " #{:<7} | Random.Range(float)    | ({:.2}, {:.2}) -> {:<15.6} | By: {}",
            self.call_counter, min, max, val, caller
        );
    }

    fn log_random_range_int(&mut self, min: i32, max: i32, val: i32, caller: &str) {
        self.call_counter += 1;
        println!(
            " #{:<7} | Random.Range(int)      | ({}, {}) -> {:<20} | By: {}",
            self.call_counter, min, max, val, caller
        );
    }

    pub fn generate_tile(
        &mut self,
        tile_gen_config: &TileGenConfiguration,
        quest_config: &QuestSystemConfiguration,
        quest_manager: &mut QuestManager,
        overwrite_quest_prob: Option<f32>,
    ) -> GeneratedTileInfo {
        let sub_seed = self.tile_generation_seed.wrapping_add(
            (self.generated_tile_count as i32)
                .wrapping_mul(self.tile_seed_increment_step),
        );

        self.generated_tile_count += 1;

        let filter = if self.generated_tile_count <= self.at_least_two_empty_edges_for_x_turns {
            TileGenFilter::AtLeastTwoEmptyEdges
        } else {
            TileGenFilter::None
        };

        if self.generated_tile_count >= 1 {
            let roll_seed = self.tile_generation_seed.wrapping_add(
                (self.generated_tile_count as i32 - 1)
                    .wrapping_mul(self.tile_seed_increment_step),
            );
            self.log_init_state(roll_seed, "TileGenerator.DMD<TileGenerator::GenerateTile>()");

            let mut prob_rng = UnityRandom::new(roll_seed);
            let _set_seed_roll = prob_rng.value();
            let rand_val: f32 = prob_rng.value();
            self.log_random_value(rand_val, "TileGenerator.DMD<TileGenerator::GenerateTile>()");

            let is_quest = match overwrite_quest_prob {
                Some(p) if p >= 0.0 => rand_val <= p,
                _ => quest_manager.should_generate_quest(quest_config, self.generated_tile_count, rand_val),
            };

            if is_quest {
                self.generated_quest_count += 1;
                quest_manager.add_quest();

                // Dùng Tile Seed (sub_seed) * 2 theo đúng Dorfromantik2.cs:L24416 và RAM Dump log!
                let quest_init_seed = sub_seed.wrapping_mul(2);

                self.log_init_state(
                    quest_init_seed,
                    "QuestSystemConfiguration.GetRandomQuestTile()",
                );

                let mut quest_rng = UnityRandom::new(quest_init_seed);
                let (_cat, quest_prefab_name) = quest_config.get_random_quest_tile(&mut quest_rng);

                let randomize_seed = self.tile_generation_seed.wrapping_add(
                    (self.generated_tile_count as i32)
                        .wrapping_mul(self.tile_seed_increment_step)
                        .wrapping_add(9441133),
                );
                self.log_init_state(randomize_seed, "Randomizer.RandomizeSeed()");

                return GeneratedTileInfo {
                    tile_index: self.generated_tile_count,
                    tile_seed: quest_init_seed,
                    is_quest: true,
                    preset_name: quest_prefab_name,
                    occupied_edges: 2,
                    rotation: 0,
                };
            }
        }

        // 2. Roll Bài Thường (GenerateBaseTile)
        self.log_init_state(sub_seed, "TileGenerator.DMD<TileGenerator::GenerateTile>()");

        let mut tile_rng = UnityRandom::new(sub_seed);
        let filtered_presets = tile_gen_config.get_filtered_tile_presets(filter);

        let selected_preset = if !filtered_presets.is_empty() {
            let total_weight: f32 = filtered_presets.iter().map(|p| p.raw_probability).sum();
            let roll_weighted = tile_rng.range_float(0.0, total_weight);
            self.log_random_range_float(0.0, 1.0, roll_weighted / total_weight, "Randomizer.SelectWeightedRandom()");

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
            let roll_weighted = tile_rng.range_float(0.0, 1.0);
            self.log_random_range_float(0.0, 1.0, roll_weighted, "Randomizer.SelectWeightedRandom()");
            &tile_gen_config.all_tile_presets[0]
        };

        let rot: i32 = tile_rng.range_int(0, 6);
        self.log_random_range_int(0, 6, rot, "Random.DMD<UnityEngine.Random::Range>()");

        for (seg_idx, _segment) in selected_preset.segment_probabilities.iter().enumerate() {
            if seg_idx > 0 {
                let max_fit_rot = (6 - seg_idx) as i32;
                if max_fit_rot > 0 {
                    let fit_rot = tile_rng.range_int(0, max_fit_rot);
                    self.log_random_range_int(0, max_fit_rot, fit_rot, "Random.DMD<UnityEngine.Random::Range>()");
                }
            }

            let seg_val: f32 = tile_rng.range_float(0.0, 1.0);
            self.log_random_range_float(0.0, 1.0, seg_val, "Randomizer.SelectWeightedRandom()");

            let hybrid_val: f32 = tile_rng.value();
            self.log_random_value(hybrid_val, "TileGenerator.DMD<TileGenerator::GenerateTile>()");
        }

        self.log_init_state(1690045082, "Randomizer.RandomizeSeed()");

        GeneratedTileInfo {
            tile_index: self.generated_tile_count,
            tile_seed: sub_seed,
            is_quest: false,
            preset_name: selected_preset.name.clone(),
            occupied_edges: selected_preset.occupied_edges,
            rotation: rot as usize,
        }
    }
}
