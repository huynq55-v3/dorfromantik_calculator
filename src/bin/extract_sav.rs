use dorfromantik_calculator::GameConfig;

fn main() {
    let save_file = "AutoSave_MonthlyMode.sav";
    let config_file = "game_config.cfg";

    let config = GameConfig::load(save_file, config_file)
        .unwrap_or_else(|err| panic!("Failed to load configuration: {}", err));

    println!("=== DORFROMANTIK GAME SESSION DATA ===");
    println!("REAL_TILE_SEED={}", config.seed);
    println!("CONFIG_STRING={}", config.config_string);
    println!();

    let save_data = &config.raw_save_data;
    let cfg = &config.raw_config_file;

    let stack_level = save_data.get_level_index(10, 6, None);
    let limit_level = save_data.get_level_index(11, 7, None);
    let density_level = save_data.get_level_index(12, 8, None);
    let quest_prob_level = save_data.get_level_index(13, 9, None);

    let quest_diff_level = save_data.get_level_index(14, 0, Some(1));
    let flag_quest_level = save_data.get_level_index(15, 0, Some(2));
    let border_level = save_data.get_level_index(16, 0, Some(3));

    let active_tile_stack_height = cfg.tile_gen.get_value(
        &cfg.world.tile_stack_height.iter().map(|v| match v {
            toml::Value::Float(f) => *f as f32,
            toml::Value::Integer(i) => *i as f32,
            _ => f32::INFINITY,
        }).collect::<Vec<f32>>(),
        stack_level,
    );
    let active_tile_limit = cfg.tile_gen.get_value(&cfg.world.tile_limit, limit_level);
    let active_density = cfg.tile_gen.get_value(&cfg.tile_gen.density, density_level);
    let active_quest_prob = cfg.tile_gen.get_value(&cfg.quest_system.quest_prob, quest_prob_level);
    let active_quest_diff = cfg.tile_gen.get_value(&cfg.quest_system.quest_difficulty, quest_diff_level);
    let active_flag_quest_prob = cfg.tile_gen.get_value(&cfg.quest_system.flag_quest_prob, flag_quest_level);

    let raw_world_border = cfg.tile_gen.get_value(&cfg.world.world_border_radius, border_level);
    let active_world_border = if raw_world_border < 0.0 { 0.0 } else { raw_world_border };

    println!("=== ACTIVE SESSION EXTRACTED VALUES ===");
    for group_cfg in &config.tile_gen.global_group_type_probabilities {
        println!("ACTIVE_{}Probability={}", group_cfg.group_type, group_cfg.raw_probability);
    }

    println!("ACTIVE_TileStackHeight={}", active_tile_stack_height);
    println!("ACTIVE_TileLimit={}", active_tile_limit);
    println!("ACTIVE_Density={}", active_density);
    println!("ACTIVE_QuestProbability={}", active_quest_prob);
    println!("ACTIVE_QuestDifficulty={}", active_quest_diff);
    println!("ACTIVE_FlagQuestProbability={}", active_flag_quest_prob);
    println!("ACTIVE_WorldBorderRadius={}", active_world_border);
}
