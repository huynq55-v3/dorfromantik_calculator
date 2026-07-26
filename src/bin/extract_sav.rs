use std::collections::HashMap;
use std::fs;

// C# NumberSystemConverter Implementation
pub struct NumberSystemConverter {
    unicode_letters: Vec<char>,
    number_base: usize,
}

impl NumberSystemConverter {
    pub fn new() -> Self {
        let excluded: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let unicode_areas: Vec<(u32, u32)> = vec![(48, 57), (65, 90), (97, 122)];

        let mut unicode_letters = Vec::new();
        for (start, end) in unicode_areas {
            for code in start..=end {
                if let Some(ch) = std::char::from_u32(code) {
                    if !excluded.contains(&ch) {
                        unicode_letters.push(ch);
                    }
                }
            }
        }
        let number_base = unicode_letters.len();

        Self {
            unicode_letters,
            number_base,
        }
    }

    pub fn decode_number_as_long(&self, encoded_number: &str, number_can_be_negative: bool) -> i64 {
        let mut num: i64 = 0;
        let chars: Vec<char> = encoded_number.chars().collect();
        let len = chars.len();

        for (i, &ch) in chars.iter().enumerate() {
            let num2 = self.unicode_letters.iter().position(|&c| c == ch).unwrap_or(0) as i64;
            let exponent = (len - 1 - i) as u32;
            let power = (self.number_base as i64).pow(exponent);
            num += num2 * power;
        }

        if number_can_be_negative {
            num = num - 2147483647 - 1;
        }
        num
    }

    pub fn decode_number_as_digits(&self, encoded_number: &str, new_base: i64) -> Vec<usize> {
        let mut val = self.decode_number_as_long(encoded_number, false);
        let mut digits = Vec::new();

        if val == 0 {
            digits.push(0);
        } else {
            while val > 0 {
                digits.push((val % new_base) as usize);
                val /= new_base;
            }
            digits.reverse();
        }

        while digits.len() < 10 {
            digits.insert(0, 0);
        }

        digits
    }
}

// Bảng RULE load động từ file config
#[derive(Debug, Clone, Default)]
pub struct RuleTable {
    pub rules: HashMap<String, Vec<f32>>,
}

impl RuleTable {
    pub fn load_from_config(config_path: &str) -> Self {
        let mut rules = HashMap::new();
        let content = fs::read_to_string(config_path)
            .unwrap_or_else(|_| panic!("Failed to read config file: {}", config_path));

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, vals_str)) = line.split_once('=') {
                let key = key.trim();
                let values: Vec<f32> = vals_str
                    .split(',')
                    .map(|v| {
                        let v = v.trim();
                        if v.eq_ignore_ascii_case("infinity") || v.eq_ignore_ascii_case("inf") {
                            f32::INFINITY
                        } else {
                            v.parse::<f32>().unwrap_or(0.0)
                        }
                    })
                    .collect();
                rules.insert(key.to_string(), values);
            }
        }

        Self { rules }
    }

    pub fn get_value(&self, rule_name: &str, level: usize) -> f32 {
        if let Some(arr) = self.rules.get(rule_name) {
            if level < arr.len() {
                return arr[level];
            }
        }
        0.0
    }
}

fn extract_session_data_from_save(save_path: &str) -> (i32, String, HashMap<i32, usize>) {
    let data = fs::read(save_path).unwrap_or_else(|_| panic!("Failed to read save file: {}", save_path));

    // 1. Trích xuất ConfigString (chuỗi 18 ký tự ASCII sau trường configString)
    let target_field = b"configString";
    let field_pos = data
        .windows(target_field.len())
        .position(|w| w == target_field)
        .unwrap_or_else(|| panic!("Could not find configString field in save file"));

    let search_window = &data[field_pos..];
    let string_pos = search_window
        .windows(18)
        .position(|w| w.iter().all(|&b| b.is_ascii_alphanumeric()))
        .unwrap_or_else(|| panic!("Could not find 18-char ASCII ConfigString after configString field"))
        + field_pos;

    let config_string = std::str::from_utf8(&data[string_pos..string_pos + 18])
        .unwrap()
        .to_string();

    // 2. Trích xuất REAL_TILE_SEED (4-byte i32 Little-Endian liền trước chuỗi ConfigString)
    let seed_bytes = &data[string_pos - 10..string_pos - 6];
    let real_tile_seed = i32::from_le_bytes(seed_bytes.try_into().unwrap());

    // 3. Trích xuất mảng cấp độ (CustomRuleData) trực tiếp từ stream BinaryFormatter save file
    let mut custom_rules_map = HashMap::new();
    let enum_marker = b"Dorfromantik.CustomRuleType";

    if let Some(enum_pos) = data.windows(enum_marker.len()).position(|w| w == enum_marker) {
        let start_pos = enum_pos + 87; // Vị trí phần tử CustomRuleData đầu tiên
        for i in 0..12 {
            let item_pos = start_pos + i * 26; // Stride 26 byte giữa các phần tử
            if item_pos + 8 <= data.len() {
                let rule_type = i32::from_le_bytes(data[item_pos..item_pos + 4].try_into().unwrap());
                let level = i32::from_le_bytes(data[item_pos + 4..item_pos + 8].try_into().unwrap());
                if (1..=16).contains(&rule_type) {
                    custom_rules_map.insert(rule_type, level as usize);
                }
            }
        }
    }

    (real_tile_seed, config_string, custom_rules_map)
}

fn main() {
    let save_file = "AutoSave_MonthlyMode.sav";
    let cfg_file = "rule_table.cfg";

    // 1. Trích xuất động dữ liệu từ file binary save game
    let (real_tile_seed, config_string, custom_rules_map) = extract_session_data_from_save(save_file);

    println!("=== DORFROMANTIK GAME SESSION DATA ===");
    println!("REAL_TILE_SEED={}", real_tile_seed);
    println!("CONFIG_STRING={}", config_string);
    println!();

    // 2. Load Bảng RULE động từ rule_table.cfg và giải mã ConfigString
    let rules = RuleTable::load_from_config(cfg_file);
    let converter = NumberSystemConverter::new();

    // Decode ConfigString theo đúng thuật toán C# DecodeNumberAsDigits trong Dorfromantik.cs
    let part2_str = &config_string[6..12];
    let part3_str = &config_string[12..18];

    let list = converter.decode_number_as_digits(part2_str, 10);
    let list2 = converter.decode_number_as_digits(part3_str, 10);

    // Tra level index: Ưu tiên lấy từ CustomRuleData trong save file hoặc decoded digit từ ConfigString
    let get_level_index = |rule_type_id: i32, decoded_digit_level: usize| -> usize {
        if let Some(&lvl) = custom_rules_map.get(&rule_type_id) {
            lvl
        } else if decoded_digit_level == 0 {
            9 // C# GetProbabilityByLevel fallback default level
        } else {
            decoded_digit_level
        }
    };

    let village_level = get_level_index(1, list[1]);
    let forest_level = get_level_index(2, list[2]);
    let agri_level = get_level_index(3, list[3]);
    let water_level = get_level_index(4, list[4]);
    let train_level = get_level_index(5, list[5]);
    let stack_level = get_level_index(10, list[6]);
    let limit_level = get_level_index(11, list[7]);
    let density_level = get_level_index(12, list[8]);
    let quest_prob_level = get_level_index(13, list[9]);

    let quest_diff_level = get_level_index(14, list2[1]);
    let flag_quest_level = get_level_index(15, list2[2]);
    let border_level = get_level_index(16, list2[3]);

    // 3. Tra con số Cấp độ (Level Index) vào mảng RULE_* vừa load từ config để tính ra ACTIVE_*
    let active_village = rules.get_value("RULE_VillageProbability", village_level);
    let active_forest = rules.get_value("RULE_ForestProbability", forest_level);
    let active_agriculture = rules.get_value("RULE_AgricultureProbability", agri_level);
    let active_water = rules.get_value("RULE_WaterProbability", water_level);
    let active_train_track = rules.get_value("RULE_TrainTrackProbability", train_level);
    let active_tile_stack_height = rules.get_value("RULE_TileStackHeight", stack_level);
    let active_tile_limit = rules.get_value("RULE_TileLimit", limit_level);
    let active_density = rules.get_value("RULE_Density", density_level);
    let active_quest_prob = rules.get_value("RULE_QuestProbability", quest_prob_level);

    let active_quest_diff = rules.get_value("RULE_QuestDifficulty", quest_diff_level);
    let active_flag_quest_prob = rules.get_value("RULE_FlagQuestProbability", flag_quest_level);
    
    // Nếu level = 0 (như trường hợp WorldBorderRadius = -1 level 0) thì in 0 theo chuẩn hiển thị
    let raw_world_border = rules.get_value("RULE_WorldBorderRadius", border_level);
    let active_world_border = if raw_world_border < 0.0 { 0.0 } else { raw_world_border };

    println!("=== ACTIVE SESSION ALL 12 EXACT EXTRACTED VALUES ===");
    println!("ACTIVE_VillageProbability={}", active_village);
    println!("ACTIVE_ForestProbability={}", active_forest);
    println!("ACTIVE_AgricultureProbability={}", active_agriculture);
    println!("ACTIVE_WaterProbability={}", active_water);
    println!("ACTIVE_TrainTrackProbability={}", active_train_track);
    println!("ACTIVE_TileStackHeight={}", active_tile_stack_height);
    println!("ACTIVE_TileLimit={}", active_tile_limit);
    println!("ACTIVE_Density={}", active_density);
    println!("ACTIVE_QuestProbability={}", active_quest_prob);
    println!("ACTIVE_QuestDifficulty={}", active_quest_diff);
    println!("ACTIVE_FlagQuestProbability={}", active_flag_quest_prob);
    println!("ACTIVE_WorldBorderRadius={}", active_world_border);
}
