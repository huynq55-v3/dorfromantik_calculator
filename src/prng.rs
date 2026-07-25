use crate::tile::{Quest, QuestType, SegmentType, Tile};
use rand::Rng;
use rand_pcg::Pcg32;
use rand::SeedableRng;

#[derive(Debug, Clone)]
pub struct CustomRules {
    pub tile_limit: Option<u32>,        // e.g. Some(250) for Monthly Mode
    pub village_weight: f64,
    pub forest_weight: f64,
    pub agriculture_weight: f64,
    pub water_weight: f64,
    pub train_weight: f64,
    pub quest_probability: f64,         // 0.0 to 1.0
}

impl Default for CustomRules {
    fn default() -> Self {
        Self {
            tile_limit: None,            // Classic infinite deck mode
            village_weight: 1.0,
            forest_weight: 1.0,
            agriculture_weight: 1.0,
            water_weight: 1.0,
            train_weight: 1.0,
            quest_probability: 0.20,
        }
    }
}

pub struct TileDeckGenerator {
    rng: Pcg32,
    pub seed: u64,
    pub rules: CustomRules,
}

impl TileDeckGenerator {
    pub fn new(seed: u64, rules: CustomRules) -> Self {
        Self {
            rng: Pcg32::seed_from_u64(seed),
            seed,
            rules,
        }
    }

    /// Predefined preset patterns for edge segments
    const PRESETS: &'static [([SegmentType; 6], SegmentType)] = &[
        // (edges, primary_type)
        ([SegmentType::Grass; 6], SegmentType::Grass),
        ([SegmentType::Forest, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Forest),
        ([SegmentType::Forest, SegmentType::Forest, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Forest),
        ([SegmentType::Forest, SegmentType::Forest, SegmentType::Forest, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Forest),
        ([SegmentType::Village, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Village),
        ([SegmentType::Village, SegmentType::Village, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Village),
        ([SegmentType::Agriculture, SegmentType::Grass, SegmentType::Agriculture, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Agriculture),
        ([SegmentType::Agriculture, SegmentType::Agriculture, SegmentType::Agriculture, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Agriculture),
        ([SegmentType::Water, SegmentType::Grass, SegmentType::Grass, SegmentType::Water, SegmentType::Grass, SegmentType::Grass], SegmentType::Water),
        ([SegmentType::Water, SegmentType::Water, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Water),
        ([SegmentType::Train, SegmentType::Grass, SegmentType::Grass, SegmentType::Train, SegmentType::Grass, SegmentType::Grass], SegmentType::Train),
        ([SegmentType::Train, SegmentType::Train, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Train),
        ([SegmentType::Forest, SegmentType::Forest, SegmentType::Village, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], SegmentType::Forest),
        ([SegmentType::Agriculture, SegmentType::Agriculture, SegmentType::Water, SegmentType::Grass, SegmentType::Water, SegmentType::Grass], SegmentType::Agriculture),
    ];

    pub fn draw_next_tile(&mut self) -> Tile {
        // Weighted random selection based on CustomRules
        let weights: Vec<f64> = Self::PRESETS
            .iter()
            .map(|(_, primary)| match primary {
                SegmentType::Grass => 1.0,
                SegmentType::Village => self.rules.village_weight,
                SegmentType::Forest => self.rules.forest_weight,
                SegmentType::Agriculture => self.rules.agriculture_weight,
                SegmentType::Water => self.rules.water_weight,
                SegmentType::Train => self.rules.train_weight,
            })
            .collect();

        let total_weight: f64 = weights.iter().sum();
        let mut r = self.rng.gen_range(0.0..total_weight);
        let mut chosen_idx = 0;

        for (idx, &w) in weights.iter().enumerate() {
            if r <= w {
                chosen_idx = idx;
                break;
            }
            r -= w;
        }

        let edges = Self::PRESETS[chosen_idx].0;

        // Quest probability check
        let has_quest = self.rng.gen_bool(self.rules.quest_probability.clamp(0.0, 1.0));
        let quest = if has_quest {
            let non_grass: Vec<SegmentType> = edges.iter().cloned().filter(|&s| s != SegmentType::Grass).collect();
            let target_type = if !non_grass.is_empty() {
                non_grass[self.rng.gen_range(0..non_grass.len())]
            } else {
                match self.rng.gen_range(0..3) {
                    0 => SegmentType::Forest,
                    1 => SegmentType::Village,
                    _ => SegmentType::Agriculture,
                }
            };

            let quest_type = if self.rng.gen_bool(0.7) {
                QuestType::MoreThan
            } else {
                QuestType::Exactly
            };

            let target_count = match target_type {
                SegmentType::Forest => self.rng.gen_range(10..=30),
                SegmentType::Village => self.rng.gen_range(5..=20),
                SegmentType::Agriculture => self.rng.gen_range(8..=25),
                SegmentType::Water => self.rng.gen_range(4..=12),
                SegmentType::Train => self.rng.gen_range(4..=12),
                SegmentType::Grass => 10,
            };

            Some(Quest {
                target_type,
                quest_type,
                target_count,
                is_fulfilled: false,
                is_flag: false,
            })
        } else {
            None
        };

        Tile::new(edges, quest)
    }
}
