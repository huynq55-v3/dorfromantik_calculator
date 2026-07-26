use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupTypeConfiguration {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub group_type: serde_json::Value,
    #[serde(default)]
    pub raw_probability: f32,
    #[serde(default)]
    pub probability_in_percent: f32,
    #[serde(default, rename = "_displayProbability")]
    pub display_probability: f32,
}

impl GroupTypeConfiguration {
    pub fn new<T: Serialize>(group_type: T, raw_probability: f32) -> Self {
        Self {
            name: String::new(),
            group_type: serde_json::to_value(group_type).unwrap_or(serde_json::Value::Null),
            raw_probability,
            probability_in_percent: 0.0,
            display_probability: 0.0,
        }
    }
}
