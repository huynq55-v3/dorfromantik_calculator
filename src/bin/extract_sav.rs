use dorfromantik_calculator::config::{CustomRuleLevelConfiguration, CustomRuleType};
use dorfromantik_calculator::utils::save_decoder::extract_session_data_from_save;
use std::fs;

fn main() {
    let save_file = "AutoSave_MonthlyMode.sav";
    let rule_table_asset_path = "assets/CustomModeLevels_Default.json";

    println!("=== DORFROMANTIK GAME SESSION DATA EXTRACTION ===");

    let rule_table = match fs::read_to_string(rule_table_asset_path) {
        Ok(content) => CustomRuleLevelConfiguration::load_from_asset_json(&content)
            .unwrap_or_else(|e| panic!("Failed to parse RuleTable JSON: {}", e)),
        Err(e) => panic!("Failed to read {}: {}", rule_table_asset_path, e),
    };

    let save_data = extract_session_data_from_save(save_file)
        .unwrap_or_else(|e| panic!("Failed to parse save file: {}", e));

    println!("REAL_TILE_SEED={}", save_data.real_tile_seed);
    println!("CONFIG_STRING={}", save_data.config_string);
    println!();

    let village_level = save_data.get_level_index(1, 1, None);
    let forest_level = save_data.get_level_index(2, 2, None);
    let agri_level = save_data.get_level_index(3, 3, None);
    let water_level = save_data.get_level_index(4, 4, None);
    let train_level = save_data.get_level_index(5, 5, None);

    let stack_level = save_data.get_level_index(10, 6, None);
    let limit_level = save_data.get_level_index(11, 7, None);
    let density_level = save_data.get_level_index(12, 8, None);
    let quest_prob_level = save_data.get_level_index(13, 9, None);

    let quest_diff_level = save_data.get_level_index(14, 0, Some(1));
    let flag_quest_level = save_data.get_level_index(15, 0, Some(2));
    let border_level = save_data.get_level_index(16, 0, Some(3));

    println!("=== ACTIVE SESSION EXTRACTED VALUES ===");
    println!(
        "ACTIVE_VillageProbability={}",
        rule_table.get_value(CustomRuleType::VillageProbability, village_level)
    );
    println!(
        "ACTIVE_ForestProbability={}",
        rule_table.get_value(CustomRuleType::ForestProbability, forest_level)
    );
    println!(
        "ACTIVE_AgricultureProbability={}",
        rule_table.get_value(CustomRuleType::AgricultureProbability, agri_level)
    );
    println!(
        "ACTIVE_WaterProbability={}",
        rule_table.get_value(CustomRuleType::WaterProbability, water_level)
    );
    println!(
        "ACTIVE_TrainTrackProbability={}",
        rule_table.get_value(CustomRuleType::TrainTrackProbability, train_level)
    );
    println!(
        "ACTIVE_TileStackHeight={}",
        rule_table.get_value(CustomRuleType::TileStackHeight, stack_level)
    );
    println!(
        "ACTIVE_TileLimit={}",
        rule_table.get_value(CustomRuleType::TileLimit, limit_level)
    );
    println!(
        "ACTIVE_Density={}",
        rule_table.get_value(CustomRuleType::Density, density_level)
    );
    println!(
        "ACTIVE_QuestProbability={}",
        rule_table.get_value(CustomRuleType::QuestProbability, quest_prob_level)
    );
    println!(
        "ACTIVE_QuestDifficulty={}",
        rule_table.get_value(CustomRuleType::QuestDifficulty, quest_diff_level)
    );
    println!(
        "ACTIVE_FlagQuestProbability={}",
        rule_table.get_value(CustomRuleType::FlagQuestProbability, flag_quest_level)
    );
    println!(
        "ACTIVE_WorldBorderRadius={}",
        rule_table.get_value(CustomRuleType::WorldBorderRadius, border_level)
    );
}
