use dorfromantik_calculator::config::TileGenConfiguration;
use std::fs;

fn main() {
    let asset_path = "assets/DefaultTileGenConfiguration.json";

    println!("Loading TileGenConfiguration directly from Unity JSON Asset Dump...");

    match fs::read_to_string(asset_path) {
        Ok(json_content) => match TileGenConfiguration::load_from_asset_json(&json_content) {
            Ok(tile_gen) => {
                println!("\n=== TILE GEN CONFIGURATION ASSET LOADED SUCCESSFULLY ===");
                println!("Default Probability: {}", tile_gen.default_probability);
                println!("Auto Update        : {}", tile_gen.auto_update);
                println!(
                    "Global Group Types : {} entries",
                    tile_gen.global_group_type_probabilities.len()
                );
                println!(
                    "Segment Collections: {} collections",
                    tile_gen.segment_preset_collections.len()
                );
                println!(
                    "Tile Collections   : {} collections",
                    tile_gen.tile_preset_collections.len()
                );

                println!("\n=== FLATTENED LISTS (after update_values) ===");
                println!(
                    "All Segment Presets (Flattened): {}",
                    tile_gen.all_segment_presets.len()
                );
                println!(
                    "All Tile Presets    (Flattened): {}",
                    tile_gen.all_tile_presets.len()
                );

                println!("\n=== Sample Presets from all_tile_presets ===");
                for (i, preset) in tile_gen.all_tile_presets.iter().take(5).enumerate() {
                    println!(
                        "  [{}] Name: {:<12} | rawProb: {:<6} | occupiedEdges: {}",
                        i + 1,
                        preset.name,
                        preset.raw_probability,
                        preset.occupied_edges
                    );
                }
            }
            Err(err) => {
                eprintln!("Failed to parse Asset JSON: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Failed to read asset file {}: {}", asset_path, err);
        }
    }
}
