use dorfromantik_calculator::config::{
    CustomRuleLevelConfiguration, CustomRuleType, QuestSystemConfiguration, TileGenConfiguration,
};
use std::fs;

fn main() {
    let tile_gen_asset_path = "assets/DefaultTileGenConfiguration.json";
    let rule_table_asset_path = "assets/CustomModeLevels_Default.json";
    let quest_system_asset_path = "assets/QuestSystemConfig_Default.json";

    println!("=== LOADING ALL 3 UNITY ASSET JSON DUMPS ===");

    // 1. Load CustomRuleLevelConfiguration Asset (RuleTable)
    let rule_table = match fs::read_to_string(rule_table_asset_path) {
        Ok(content) => match CustomRuleLevelConfiguration::load_from_asset_json(&content) {
            Ok(config) => {
                println!("[✓] Loaded RuleTable Asset: CustomModeLevels_Default.json");
                config
            }
            Err(err) => panic!("Failed to parse RuleTable JSON: {}", err),
        },
        Err(err) => panic!("Failed to read {}: {}", rule_table_asset_path, err),
    };

    println!("\n=== RULE TABLE SAMPLE VALUES ===");
    let village_default_lvl = rule_table.get_default_level(CustomRuleType::VillageProbability);
    println!("Default Level for VillageProbability : Level {}", village_default_lvl);
    println!("Density at Level 9                   : {}", rule_table.get_value(CustomRuleType::Density, 9));

    // 2. Load TileGenConfiguration Asset
    let tile_gen = match fs::read_to_string(tile_gen_asset_path) {
        Ok(content) => match TileGenConfiguration::load_from_asset_json(&content) {
            Ok(config) => {
                println!("\n[✓] Loaded TileGenConfiguration Asset: DefaultTileGenConfiguration.json");
                config
            }
            Err(err) => panic!("Failed to parse TileGenConfiguration JSON: {}", err),
        },
        Err(err) => panic!("Failed to read {}: {}", tile_gen_asset_path, err),
    };

    println!("\n=== TILE GEN CONFIGURATION STATS ===");
    println!("All Segment Presets (Flattened): {}", tile_gen.all_segment_presets.len());
    println!("All Tile Presets    (Flattened): {}", tile_gen.all_tile_presets.len());

    // 3. Load QuestSystemConfiguration Asset
    let quest_system = match fs::read_to_string(quest_system_asset_path) {
        Ok(content) => match QuestSystemConfiguration::load_from_asset_json(&content) {
            Ok(config) => {
                println!("\n[✓] Loaded QuestSystemConfiguration Asset: QuestSystemConfig_Default.json");
                config
            }
            Err(err) => panic!("Failed to parse QuestSystemConfiguration JSON: {}", err),
        },
        Err(err) => panic!("Failed to read {}: {}", quest_system_asset_path, err),
    };

    println!("\n=== QUEST SYSTEM CONFIGURATION STATS ===");
    println!("Auto Update                      : {}", quest_system.auto_update);
    println!("Quest Probabilities Entries      : {}", quest_system.quest_probabilities.len());
    println!("Quest Tile Collections Count     : {}", quest_system.quest_tile_collections.len());
    println!("Quest Tile Probability (0 Quests): {}", quest_system.quest_tile_probability(0, 0));
    println!("Quest Tile Probability (1 Quest) : {}", quest_system.quest_tile_probability(1, 0));
}
