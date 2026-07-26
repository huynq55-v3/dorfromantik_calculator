use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
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

impl<'de> Deserialize<'de> for CustomRuleType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = serde_json::Value::deserialize(deserializer)?;
        let id = match val {
            serde_json::Value::Number(n) => n.as_i64().unwrap_or(0) as i32,
            serde_json::Value::String(s) => match s.as_str() {
                "VillageProbability" => 1,
                "ForestProbability" => 2,
                "AgricultureProbability" => 3,
                "WaterProbability" => 4,
                "TrainTrackProbability" => 5,
                "TileStackHeight" => 10,
                "TileLimit" => 11,
                "Density" => 12,
                "QuestProbability" => 13,
                "QuestDifficulty" => 14,
                "FlagQuestProbability" => 15,
                "WorldBorderRadius" => 16,
                _ => s.parse::<i32>().unwrap_or(0),
            },
            _ => 0,
        };

        match id {
            1 => Ok(CustomRuleType::VillageProbability),
            2 => Ok(CustomRuleType::ForestProbability),
            3 => Ok(CustomRuleType::AgricultureProbability),
            4 => Ok(CustomRuleType::WaterProbability),
            5 => Ok(CustomRuleType::TrainTrackProbability),
            10 => Ok(CustomRuleType::TileStackHeight),
            11 => Ok(CustomRuleType::TileLimit),
            12 => Ok(CustomRuleType::Density),
            13 => Ok(CustomRuleType::QuestProbability),
            14 => Ok(CustomRuleType::QuestDifficulty),
            15 => Ok(CustomRuleType::FlagQuestProbability),
            16 => Ok(CustomRuleType::WorldBorderRadius),
            _ => Err(serde::de::Error::custom(format!("Unknown CustomRuleType id: {}", id))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomRuleData {
    pub rule_type: CustomRuleType,
    pub value: usize,
}

impl CustomRuleData {
    pub fn new(rule_type: CustomRuleType, value: usize) -> Self {
        Self { rule_type, value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomModeLevelProbabilities {
    pub rule_type: CustomRuleType,
    pub probability_by_level: Vec<serde_json::Value>,
}

impl CustomModeLevelProbabilities {
    pub fn get_value(&self, level: usize) -> f32 {
        if level < self.probability_by_level.len() {
            match &self.probability_by_level[level] {
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
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomRuleLevelConfigurationData {
    #[serde(default)]
    pub default_levels: Vec<CustomRuleData>,
    #[serde(default)]
    pub probability_by_level: Vec<CustomModeLevelProbabilities>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRuleLevelConfigurationAsset {
    #[serde(rename = "m_Name", default)]
    pub name: String,
    #[serde(rename = "m_Structure")]
    pub structure: CustomRuleLevelConfigurationData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomRuleLevelConfiguration {
    pub default_levels: Vec<CustomRuleData>,
    pub probability_by_level: Vec<CustomModeLevelProbabilities>,
}

impl CustomRuleLevelConfiguration {
    pub fn load_from_asset_json(json_str: &str) -> Result<Self, String> {
        let asset: CustomRuleLevelConfigurationAsset = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse CustomRuleLevelConfiguration JSON asset: {}", e))?;

        Ok(Self {
            default_levels: asset.structure.default_levels,
            probability_by_level: asset.structure.probability_by_level,
        })
    }

    pub fn get_default_level(&self, rule_type: CustomRuleType) -> usize {
        self.default_levels
            .iter()
            .find(|x| x.rule_type == rule_type)
            .map(|x| x.value)
            .unwrap_or(9)
    }

    pub fn get_value(&self, rule_type: CustomRuleType, level: usize) -> f32 {
        if let Some(rule) = self.probability_by_level.iter().find(|x| x.rule_type == rule_type) {
            rule.get_value(level)
        } else {
            0.0
        }
    }
}
