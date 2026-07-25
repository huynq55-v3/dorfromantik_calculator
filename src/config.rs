use crate::prng::CustomRules;

const BASE62_CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn decode_base62(encoded: &str) -> u64 {
    let mut val: u64 = 0;
    for &b in encoded.as_bytes() {
        if let Some(idx) = BASE62_CHARS.iter().position(|&c| c == b) {
            val = val * 62 + idx as u64;
        }
    }
    val
}

pub fn decode_digits_10(num: u64, target_len: usize) -> Vec<u32> {
    let s = num.to_string();
    let mut digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    while digits.len() < target_len {
        digits.insert(0, 0);
    }
    digits
}

/// Decodes an 18-character ConfigString like "0720262fJmCw2gRsn6" into CustomRules
pub fn parse_config_string(config_str: &str) -> (u64, CustomRules) {
    let clean = config_str.replace("-", "");
    if clean.len() < 18 {
        return (3103784960, CustomRules::default());
    }

    // Part 1: Seed / Month-Year (first 6 chars)
    let seed_str = &clean[0..6];
    let seed = decode_base62(seed_str);

    // Part 2: Rules Part 1 (chars 6..12 e.g. "2fJmCw")
    let part1_val = decode_base62(&clean[6..12]);
    let digits1 = decode_digits_10(part1_val, 10);

    // Part 3: Rules Part 2 (chars 12..18 e.g. "2gRsn6")
    let part2_val = decode_base62(&clean[12..18]);
    let digits2 = decode_digits_10(part2_val, 10);

    let forest_level = digits1[2] as f64;
    let agri_level = digits1[3] as f64;
    let water_level = digits1[4] as f64;
    let train_level = digits1[5] as f64;
    let tile_limit_level = digits1[7];
    let quest_prob_level = digits1[9] as f64;

    let flag_quest_level = digits2[2] as f64;

    // Level 4 in TileLimit corresponds to 250 Tiles
    let tile_limit = match tile_limit_level {
        1 => Some(50),
        2 => Some(100),
        3 => Some(150),
        4 => Some(250),
        5 => Some(500),
        _ => Some(250),
    };

    let rules = CustomRules {
        tile_limit,
        village_weight: 1.0,
        forest_weight: 1.0 + (forest_level * 0.15),
        agriculture_weight: 1.0 + (agri_level * 0.15),
        water_weight: 1.0 + (water_level * 0.15),
        train_weight: 1.0 + (train_level * 0.15),
        quest_probability: 0.15 + (quest_prob_level * 0.03) + (flag_quest_level * 0.02),
    };

    (seed, rules)
}
