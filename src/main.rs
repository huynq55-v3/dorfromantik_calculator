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

    println!("=========================================================================================");
    println!(" DORFROMANTIK EXACT QUEST TILE TOPOLOGY & ROTATION LOOKAHEAD PREDICTION REPORT");
    println!("=========================================================================================\n");

    let rule_table = CustomRuleLevelConfiguration::load_from_asset_json(
        &fs::read_to_string(rule_table_asset_path).unwrap(),
    )
    .unwrap();

    let meta_db = MetadataDatabase::load_from_file(metadata_asset_path);
    let save_data = extract_session_data_from_save(save_file).unwrap();
    let master_seed = save_data.real_tile_seed; // -2099861831
    let step = -73111;

    println!("📌 [CẤU HÌNH HẠT GIỐNG CHÍNH VÁN CHƠI]:");
    println!(" • Master Seed (Save Game)   : {}", master_seed);
    println!(" • Tile Seed Increment Step  : {}\n", step);

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

    println!("-----------------------------------------------------------------------------------------------------------------------");
    println!(" 🎴 DỰ ĐOÁN 100% CHÍNH XÁC CẤU TRÚC 6 CẠNH, 6 HOÁN VỊ XOAY VÀ SỐ VẬT THỂ CHO TỪNG QUEST TILE:");
    println!("-----------------------------------------------------------------------------------------------------------------------");
    for k in 0..30 {
        let quest_seed = master_seed.wrapping_add((k as i32).wrapping_mul(step));
        let quest_init_seed = quest_seed.wrapping_mul(2);
        let mut quest_rng = UnityRandom::new(quest_init_seed);
        let filter = if k < 5 {
            TileGenFilter::AtLeastTwoEmptyEdges
        } else {
            TileGenFilter::None
        };
        let (_cat_id, quest_sub_name) =
            quest_config.get_random_quest_tile_filtered(&mut quest_rng, filter);

        let topo = meta_db.get_topology(&quest_sub_name);

        println!(
            "  • Quest #{:02} | Seed: {:11} | Cạnh Chuẩn: {:11} | Tóm Tắt: {:12} | Vật Thể: {:32} | SubCol: \"{}\"",
            k + 1,
            quest_seed,
            topo.format_edges(),
            topo.summary,
            topo.format_exact_objects(),
            quest_sub_name
        );
        let rots = topo.format_rotations();
        println!("     └─► 6 Hoán vị xoay (Rotations 0..5): [{}] | [{}] | [{}] | [{}] | [{}] | [{}]",
            rots[0], rots[1], rots[2], rots[3], rots[4], rots[5]
        );
    }
    println!("-----------------------------------------------------------------------------------------------------------------------\n");
}
