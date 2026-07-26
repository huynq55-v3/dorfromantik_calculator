use dorfromantik_calculator::GameConfig;

fn main() {
    let save_file = "AutoSave_MonthlyMode.sav";
    let config_file = "game_config.cfg";

    println!("Loading configuration using GameConfig loader pattern...");

    match GameConfig::load(save_file, config_file) {
        Ok(config) => {
            println!("\n=== GAME SESSION CONFIGURATION LOADED ===");
            println!("REAL_TILE_SEED: {}", config.seed);
            println!("CONFIG_STRING  : {}", config.config_string);

            println!("\n=== TileGenConfiguration ===");
            println!("globalGroupTypeProbabilities:");
            for group_cfg in &config.tile_gen.global_group_type_probabilities {
                println!(
                    "  - {:<12} : rawProbability = {}",
                    group_cfg.group_type.to_string(),
                    group_cfg.raw_probability
                );
            }
        }
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
        }
    }
}
