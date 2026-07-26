use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomRuleType {
    VillageProbability = 1,
    ForestProbability = 2,
    AgricultureProbability = 3,
    WaterProbability = 4,
    TrainTrackProbability = 5,
    TileStackHeight = 10,
    TileLimit = 11,
    Density = 12,
    QuestProbability = 13,
    QuestDifficulty = 14,
    FlagQuestProbability = 15,
    WorldBorderRadius = 16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomModeLevelProbabilities {
    pub rule_type: CustomRuleType,
    pub probability_by_level: Vec<f32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomRuleLevelConfiguration {
    pub probability_by_level: Vec<CustomModeLevelProbabilities>,
}

impl CustomRuleLevelConfiguration {
    pub fn load_from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse RuleTable JSON: {}", e))
    }

    pub fn get_value(&self, rule_type: CustomRuleType, level: usize) -> f32 {
        if let Some(rule) = self.probability_by_level.iter().find(|x| x.rule_type == rule_type) {
            if level < rule.probability_by_level.len() {
                return rule.probability_by_level[level];
            }
        }
        0.0
    }
}
