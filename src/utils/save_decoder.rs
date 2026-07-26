use std::collections::HashMap;
use std::fs;
use crate::utils::number_converter::NumberSystemConverter;

#[derive(Debug, Clone)]
pub struct SessionSaveData {
    pub real_tile_seed: i32,
    pub config_string: String,
    pub custom_rules_map: HashMap<i32, usize>,
    pub decoded_digits_part2: Vec<usize>,
    pub decoded_digits_part3: Vec<usize>,
}

impl SessionSaveData {
    pub fn get_level_index(&self, rule_type_id: i32, part2_index: usize, part3_index: Option<usize>) -> usize {
        if let Some(&lvl) = self.custom_rules_map.get(&rule_type_id) {
            lvl
        } else {
            let digit = if let Some(p3) = part3_index {
                if p3 < self.decoded_digits_part3.len() {
                    self.decoded_digits_part3[p3]
                } else {
                    0
                }
            } else if part2_index < self.decoded_digits_part2.len() {
                self.decoded_digits_part2[part2_index]
            } else {
                0
            };
            if digit == 0 {
                9
            } else {
                digit
            }
        }
    }
}

pub fn extract_session_data_from_save(save_path: &str) -> Result<SessionSaveData, String> {
    let data = fs::read(save_path).map_err(|e| format!("Failed to read save file {}: {}", save_path, e))?;

    let target_field = b"configString";
    let field_pos = data
        .windows(target_field.len())
        .position(|w| w == target_field)
        .ok_or_else(|| "Could not find configString field in save file".to_string())?;

    let search_window = &data[field_pos..];
    let string_offset = search_window
        .windows(18)
        .position(|w| w.iter().all(|&b| b.is_ascii_alphanumeric()))
        .ok_or_else(|| "Could not find 18-char ASCII ConfigString".to_string())?;

    let string_pos = field_pos + string_offset;

    let config_string = std::str::from_utf8(&data[string_pos..string_pos + 18])
        .map_err(|e| format!("UTF-8 decode error: {}", e))?
        .to_string();

    let seed_bytes = &data[string_pos - 10..string_pos - 6];
    let real_tile_seed = i32::from_le_bytes(seed_bytes.try_into().map_err(|_| "Seed conversion error")?);

    let mut custom_rules_map = HashMap::new();
    let enum_marker = b"Dorfromantik.CustomRuleType";

    if let Some(enum_pos) = data.windows(enum_marker.len()).position(|w| w == enum_marker) {
        let start_pos = enum_pos + 87;
        for i in 0..12 {
            let item_pos = start_pos + i * 26;
            if item_pos + 8 <= data.len() {
                let rule_type = i32::from_le_bytes(data[item_pos..item_pos + 4].try_into().unwrap());
                let level = i32::from_le_bytes(data[item_pos + 4..item_pos + 8].try_into().unwrap());
                if (1..=16).contains(&rule_type) {
                    custom_rules_map.insert(rule_type, level as usize);
                }
            }
        }
    }

    let converter = NumberSystemConverter::new();
    let part2_str = &config_string[6..12];
    let part3_str = &config_string[12..18];

    let decoded_digits_part2 = converter.decode_number_as_digits(part2_str, 10);
    let decoded_digits_part3 = converter.decode_number_as_digits(part3_str, 10);

    Ok(SessionSaveData {
        real_tile_seed,
        config_string,
        custom_rules_map,
        decoded_digits_part2,
        decoded_digits_part3,
    })
}
