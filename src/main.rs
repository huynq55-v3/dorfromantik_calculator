use dorfromantik_calculator::config::{
    CustomRuleLevelConfiguration, CustomRuleType, QuestSystemConfiguration, TileGenFilter,
};
use dorfromantik_calculator::utils::save_decoder::extract_session_data_from_save;
use dorfromantik_calculator::utils::UnityRandom;
use std::fs;

fn main() {
    let save_file = "AutoSave_MonthlyMode.sav";
    let rule_table_asset_path = "assets/CustomModeLevels_Default.json";
    let quest_system_asset_path = "assets/QuestSystemConfig_Default.json";

    println!("=========================================================================================");
    println!(" DORFROMANTIK COMPREHENSIVE SEED & FLOAT MULTIPLICATION REPORT (2375.0 WEIGHT)");
    println!("=========================================================================================\n");

    let rule_table = CustomRuleLevelConfiguration::load_from_asset_json(
        &fs::read_to_string(rule_table_asset_path).unwrap(),
    )
    .unwrap();

    let save_data = extract_session_data_from_save(save_file).unwrap();
    let master_seed = save_data.real_tile_seed; // -2099861831
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

    let total_weight: f32 = 2375.0;

    println!("📌 [CẤU HÌNH HẠT GIỐNG TỪ SAVE GAME]:");
    println!(" • Master Seed (Save Game)   : {}", master_seed);
    println!(" • Tile Seed Increment Step  : {}", step);
    println!(" • Active Total Weight (num) : {} (Forest:125, Agri:125, Village:1000, Train:1000, Water:125)", total_weight);
    println!("-----------------------------------------------------------------------------------------\n");

    let expected_ram_tiles: [(&str, i32, &str); 7] = [
        ("Quest #1", -2099861831, "QuestTile_Village_2AV"),
        ("Quest #2", -2099934942, "QuestTile_Village_2AV"),
        ("Quest #3", -2100008053, "QuestTile_Village_2AV"),
        ("Quest #4", -2100081164, "QuestTile_Train_2CT_Locomotive"),
        ("Quest #5", -2100154275, "QuestTile_Village_2AV"),
        ("Quest #6", -2100227386, "QuestTile_Train_2CT-1AF-1AV_Locomotive"),
        ("Quest #7", -2100300497, "QuestTile_Village_3AV_3AF"),
    ];

    for (label, tile_seed, expected_ram) in expected_ram_tiles {
        let init_seed = tile_seed.wrapping_mul(2);
        let mut rng = UnityRandom::new(init_seed);

        let float1 = rng.value(); // x
        let y = float1 * total_weight; // y = x * 2375.0

        let (interval_str, category) = if y < 125.0 {
            ("  0.0 ..  125.0", "Forest (Rừng)")
        } else if y < 250.0 {
            ("125.0 ..  250.0", "Agriculture (Đồng Ruộng)")
        } else if y < 1250.0 {
            ("250.0 .. 1250.0", "Village (Ngôi Làng / Nhà)")
        } else if y < 2250.0 {
            ("1250.0 .. 2250.0", "TrainTrack (Đường Ray / Sắt)")
        } else {
            ("2250.0 .. 2375.0", "Water (Sông / Nước)")
        };

        let filter = if label == "Quest #4" || label == "Quest #6" {
            TileGenFilter::AtLeastTwoEmptyEdges
        } else {
            TileGenFilter::None
        };

        let mut rng_sim = UnityRandom::new(init_seed);
        let (_cat, prefab_name) = quest_config.get_random_quest_tile_filtered(&mut rng_sim, filter);

        println!("-----------------------------------------------------------------------------------------");
        println!(" {} | Tile Seed = {} | InitState Seed = {}", label, tile_seed, init_seed);
        println!("   -> Phép Nhận (x * 2375 = y) : {:.6} * 2375.0 = {:.2}", float1, y);
        println!("   -> Khoảng Chiếu Score (y)   : [{}]", interval_str);
        println!("   -> Nhóm Địa Hình Được Chọn  : {}", category);
        println!("   -> Object Prefab Sinh Ra    : {}", prefab_name);
        println!("   -> Expected Live RAM Dump   : {}", expected_ram);
    }
    println!("-----------------------------------------------------------------------------------------\n");
}
