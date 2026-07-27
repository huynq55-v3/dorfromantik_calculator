use dorfromantik_calculator::config::{
    CustomRuleLevelConfiguration, CustomRuleType, QuestSystemConfiguration, TileGenFilter,
};
use dorfromantik_calculator::core::tile_topology::MetadataDatabase;
use dorfromantik_calculator::utils::save_decoder::extract_session_data_from_save;
use dorfromantik_calculator::utils::UnityRandom;
use std::fs;

fn main() {
    let save_file = "AutoSave_MonthlyMode.sav";
    let rule_table_asset_path = "assets/CustomModeLevels_Default.json";
    let quest_system_asset_path = "assets/QuestSystemConfig_Default.json";
    let metadata_asset_path = "assets/ExactTileMetadata.json";

    println!("=== DORFROMANTIK QUEST DEEP INSPECTION LOG (RUST STANDALONE DEBUG TOOL) ===\n");

    let rule_table = CustomRuleLevelConfiguration::load_from_asset_json(
        &fs::read_to_string(rule_table_asset_path).unwrap(),
    )
    .unwrap();

    let meta_db = MetadataDatabase::load_from_file(metadata_asset_path);
    let save_data = extract_session_data_from_save(save_file).unwrap();
    let master_seed = save_data.real_tile_seed;
    let step = -73111;

    let village_lvl = save_data.get_level_index(1, 9, None);
    let forest_lvl = save_data.get_level_index(2, 2, None);
    let agri_lvl = save_data.get_level_index(3, 2, None);
    let water_lvl = save_data.get_level_index(4, 2, None);
    let train_lvl = save_data.get_level_index(5, 9, None);
    let density_lvl = save_data.get_level_index(12, 9, None);

    let active_village = rule_table.get_value(CustomRuleType::VillageProbability, village_lvl);
    let active_forest = rule_table.get_value(CustomRuleType::ForestProbability, forest_lvl);
    let active_agri = rule_table.get_value(CustomRuleType::AgricultureProbability, agri_lvl);
    let active_water = rule_table.get_value(CustomRuleType::WaterProbability, water_lvl);
    let active_train = rule_table.get_value(CustomRuleType::TrainTrackProbability, train_lvl);
    let active_density = rule_table.get_value(CustomRuleType::Density, density_lvl);

    let mut quest_config = QuestSystemConfiguration::load_from_asset_json(
        &fs::read_to_string(quest_system_asset_path).unwrap(),
    )
    .unwrap();

    quest_config.apply_session_settings(
        active_village,
        active_forest,
        active_agri,
        active_water,
        active_train,
        active_density,
        3.0,
        5.0,
        0.2,
    );

    // Deep inspection for Quest #9 (k = 8 -> Seed: -2100446719)
    let target_k = 14; // Quest #9
    let quest_seed = master_seed.wrapping_add((target_k as i32).wrapping_mul(step));
    let quest_init_seed = quest_seed.wrapping_mul(2);
    let mut rng = UnityRandom::new(quest_init_seed);

    println!("=========================================================================================");
    println!("🔍 SOI CHI TIẾT TOÀN BỘ TRẠNG THÁI CHO QUEST #09 (Seed = {})", quest_seed);
    println!("=========================================================================================\n");

    println!("📌 1. DANH SÁCH COLLECTIONS VÀ SUBCOLLECTIONS TRONG RUST ENGINE:");
    for (c_idx, col) in quest_config.quest_tile_collections.iter().enumerate() {
        let group_id = col.group_type.m_path_id;
        let group_name = match group_id {
            41478 => "Forest",
            41479 => "Agriculture",
            41480 => "TrainTracks",
            41481 => "Village",
            41482 => "Water",
            _ => "Unknown",
        };
        println!("  ────────── Collection [{}]: GroupType = {} | rawProb = {} ──────────", c_idx, group_name, col.raw_probability);
        for (s_idx, sub) in col.sub_collections.iter().enumerate() {
            println!(
                "    [{}] SubName: \"{:22}\" | Prob: {:10.4} | Edges: {} | QuestTiles: {}",
                s_idx, sub.name, sub.sub_collection_raw_probability, sub.occupied_edges, sub.quest_tiles.len()
            );
        }
    }
    println!();

    let valid_cols: Vec<_> = quest_config
        .quest_tile_collections
        .iter()
        .filter(|c| c.raw_probability > 0.0)
        .collect();

    let total_col_weight: f32 = valid_cols.iter().map(|c| c.raw_probability).sum();
    let col_roll = rng.range_float(0.0, total_col_weight);

    let mut roll = col_roll;
    let mut chosen_col = valid_cols[0];
    for c in &valid_cols {
        if roll <= c.raw_probability {
            chosen_col = c;
            break;
        }
        roll -= c.raw_probability;
    }

    let chosen_name = match chosen_col.group_type.m_path_id {
        41478 => "Forest",
        41479 => "Agriculture",
        41480 => "TrainTracks",
        41481 => "Village",
        41482 => "Water",
        _ => "Unknown",
    };

    println!("📌 2. THỰC THI THUẬT TOÁN RÚT LƯỢT 1:");
    println!(" • Tổng trọng số Collection : {}", total_col_weight);
    println!(" • Giá trị y rút ra (Lượt 1): {}", col_roll);
    println!(" • Collection chiến thắng   : {}\n", chosen_name);

    let valid_subs: Vec<_> = chosen_col
        .sub_collections
        .iter()
        .filter(|s| s.sub_collection_raw_probability > 0.0)
        .collect();

    println!("📌 3. DANH SÁCH SUBCOLLECTIONS CUỐI CÙNG TRƯỚC KHU RÚT LƯỢT 2:");
    let total_sub_weight: f32 = valid_subs.iter().map(|s| s.sub_collection_raw_probability).sum();
    for (i, s) in valid_subs.iter().enumerate() {
        println!("   [{}] Name: \"{:22}\" | Prob: {:10.4} | Edges: {}", i, s.name, s.sub_collection_raw_probability, s.occupied_edges);
    }

    let sub_roll = rng.range_float(0.0, total_sub_weight);
    println!("\n • Tổng trọng số SubCollection : {}", total_sub_weight);
    println!(" • Giá trị z rút ra (Lượt 2)    : {}", sub_roll);

    let mut roll2 = sub_roll;
    let mut chosen_sub = valid_subs[0];
    for s in &valid_subs {
        if roll2 <= s.sub_collection_raw_probability {
            chosen_sub = s;
            break;
        }
        roll2 -= s.sub_collection_raw_probability;
    }

    let topo = meta_db.get_topology(&chosen_sub.name);

    println!(" • SubCollection Chiến Thắng    : \"{}\"", chosen_sub.name);
    println!(" • Cạnh Chuẩn 6-Edge Topology  : {}", topo.format_edges());
    println!("=========================================================================================\n");
}
