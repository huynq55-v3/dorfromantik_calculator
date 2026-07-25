use crate::tile::{Quest, QuestType, SegmentType, Tile};
use rand::Rng;
use rand_pcg::Pcg32;
use rand::SeedableRng;

pub struct TileDeckGenerator {
    rng: Pcg32,
    pub seed: u64,
}

impl TileDeckGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Pcg32::seed_from_u64(seed),
            seed,
        }
    }

    /// Predefined preset patterns for edge segments
    const PRESETS: &'static [[SegmentType; 6]] = &[
        // All Grass
        [SegmentType::Grass; 6],
        // Forest patterns
        [SegmentType::Forest, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass],
        [SegmentType::Forest, SegmentType::Forest, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass],
        [SegmentType::Forest, SegmentType::Forest, SegmentType::Forest, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass],
        // Village patterns
        [SegmentType::Village, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass],
        [SegmentType::Village, SegmentType::Village, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass],
        // Agriculture patterns
        [SegmentType::Agriculture, SegmentType::Grass, SegmentType::Agriculture, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass],
        [SegmentType::Agriculture, SegmentType::Agriculture, SegmentType::Agriculture, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass],
        // Water / River patterns
        [SegmentType::Water, SegmentType::Grass, SegmentType::Grass, SegmentType::Water, SegmentType::Grass, SegmentType::Grass], // River straight
        [SegmentType::Water, SegmentType::Water, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], // River curve
        // Train track patterns
        [SegmentType::Train, SegmentType::Grass, SegmentType::Grass, SegmentType::Train, SegmentType::Grass, SegmentType::Grass], // Rail straight
        [SegmentType::Train, SegmentType::Train, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass], // Rail curve
        // Hybrid mixes
        [SegmentType::Forest, SegmentType::Forest, SegmentType::Village, SegmentType::Grass, SegmentType::Grass, SegmentType::Grass],
        [SegmentType::Agriculture, SegmentType::Agriculture, SegmentType::Water, SegmentType::Grass, SegmentType::Water, SegmentType::Grass],
    ];

    pub fn draw_next_tile(&mut self) -> Tile {
        let preset_idx = self.rng.gen_range(0..Self::PRESETS.len());
        let edges = Self::PRESETS[preset_idx];

        // 20% chance to spawn a Quest on this tile
        let has_quest = self.rng.gen_bool(0.20);
        let quest = if has_quest {
            // Pick a non-grass segment present on tile, or default to Forest/Village
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
