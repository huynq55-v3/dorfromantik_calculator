use crate::config::QuestSystemConfiguration;

#[derive(Debug, Clone, Default)]
pub struct QuestManager {
    pub active_quest_count: usize,
    pub total_quests_generated: usize,
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            active_quest_count: 0,
            total_quests_generated: 0,
        }
    }

    /// Kiểm tra xem lượt rút tới có sinh Quest Tile dựa theo đường cong tỉ lệ hay không
    pub fn should_generate_quest(
        &self,
        config: &QuestSystemConfiguration,
        total_tiles_placed: usize,
        random_value: f32,
    ) -> bool {
        let prob = config.quest_tile_probability(self.active_quest_count, total_tiles_placed);
        random_value < prob
    }

    pub fn add_quest(&mut self) {
        self.active_quest_count += 1;
        self.total_quests_generated += 1;
    }

    pub fn complete_or_remove_quest(&mut self) {
        if self.active_quest_count > 0 {
            self.active_quest_count -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.active_quest_count = 0;
        self.total_quests_generated = 0;
    }
}
