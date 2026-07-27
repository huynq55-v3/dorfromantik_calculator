use dorfromantik_calculator::config::{
    CustomRuleLevelConfiguration, CustomRuleType, QuestSystemConfiguration, TileGenConfiguration,
    TileGenFilter,
};
use dorfromantik_calculator::core::tile_topology::{compute_canonical_edges_from_name, MetadataDatabase, TerrainType};
use dorfromantik_calculator::utils::save_decoder::extract_session_data_from_save;
use dorfromantik_calculator::utils::UnityRandom;
use std::fs;

fn main() {
    let save_file = "AutoSave_MonthlyMode.sav";
    let rule_table_asset_path = "assets/CustomModeLevels_Default.json";
    let quest_system_asset_path = "assets/QuestSystemConfig_Default.json";
    let tile_gen_asset_path = "assets/DefaultTileGenConfiguration.json";
    let metadata_asset_path = "assets/ExactTileMetadata.json";

    println!("=========================================================================================");
    println!(" DORFROMANTIK DYNAMIC TILE LOOKAHEAD PREDICTION REPORT (QUEST & EXACT NORMAL TILES)");
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

    let mut tile_gen_config = TileGenConfiguration::load_from_asset_json(
        &fs::read_to_string(tile_gen_asset_path).unwrap(),
    )
    .unwrap();

    tile_gen_config.apply_session_settings(
        active_village,
        active_forest,
        active_agri,
        active_water,
        active_train,
        active_density,
    );

    println!("-----------------------------------------------------------------------------------------------------------------------");
    println!(" 🎴 DỰ ĐOÁN 100% CHÍNH XÁC DANH SÁCH QUEST TILES (QUEST #01 -> #30):");
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
            "  • Quest #{:02} | Seed: {:11} | Cạnh Chuẩn: {:11} | Tóm Tắt: {:12} | SubCol: \"{}\"",
            k + 1,
            quest_seed,
            topo.format_edges(),
            topo.summary,
            quest_sub_name
        );
        let rots = topo.format_rotations();
        println!("     └─► 6 Hoán vị xoay (Rotations 0..5): [{}] | [{}] | [{}] | [{}] | [{}] | [{}]",
            rots[0], rots[1], rots[2], rots[3], rots[4], rots[5]
        );
    }
    println!("-----------------------------------------------------------------------------------------------------------------------\n");

    println!("-----------------------------------------------------------------------------------------------------------------------");
    println!(" 🎲 DỰ ĐOÁN CHÍNH XÁC DANH SÁCH NORMAL TILES / BASE TILES (BASE TILE #01 -> #30):");
    println!("-----------------------------------------------------------------------------------------------------------------------");
    for n in 0..30 {
        let base_seed = master_seed.wrapping_add((n as i32).wrapping_mul(step));
        let mut tile_rng = UnityRandom::new(base_seed);

        let filter = TileGenFilter::None;

        let filtered_presets = tile_gen_config.get_filtered_tile_presets(filter);
        let selected_preset = if !filtered_presets.is_empty() {
            let total_weight: f32 = filtered_presets
                .iter()
                .map(|p| if p.tile_preset_probability > 0.0 { p.tile_preset_probability } else { p.raw_probability })
                .sum();
            let roll_weighted = tile_rng.range_float(0.0, total_weight);

            let mut roll = roll_weighted;
            let mut chosen = filtered_presets[0];
            for p in &filtered_presets {
                let w = if p.tile_preset_probability > 0.0 { p.tile_preset_probability } else { p.raw_probability };
                if roll <= w {
                    chosen = p;
                    break;
                }
                roll -= w;
            }
            chosen
        } else {
            &tile_gen_config.all_tile_presets[0]
        };

        let rot = tile_rng.range_int(0, 6) as usize;

        // Rút địa hình cụ thể cho từng segment của Preset theo đúng Unity C# (kèm lọc SegmentsAdjacent)
        let mut seg_tokens = Vec::new();
        let mut previous_group_codes: Vec<&str> = Vec::new();

        for seg in &selected_preset.segment_probabilities {
            let mut valid_types: Vec<_> = seg
                .possible_types
                .iter()
                .filter(|g| g.raw_probability > 0.0)
                .collect();

            // Lọc SegmentsAdjacent: Loại bỏ các loại địa hình trùng với segment kề liền trước đó
            if !previous_group_codes.is_empty() {
                valid_types.retain(|g| {
                    let code = extract_group_type_code(&g.group_type, &g.name);
                    !previous_group_codes.contains(&code)
                });
            }

            if !valid_types.is_empty() {
                let total_w: f32 = valid_types.iter().map(|g| g.raw_probability).sum();
                let roll_w = tile_rng.range_float(0.0, total_w);
                let mut r = roll_w;
                let mut chosen_g = valid_types[0];
                for g in &valid_types {
                    if r <= g.raw_probability {
                        chosen_g = g;
                        break;
                    }
                    r -= g.raw_probability;
                }

                let g_code = extract_group_type_code(&chosen_g.group_type, &chosen_g.name);
                previous_group_codes.push(g_code);
                let seg_type_str = extract_seg_type_name(&seg.segment_type);
                seg_tokens.push(format!("{}{}", seg_type_str, g_code));
            }
        }

        let dynamic_tile_name = seg_tokens.join(" ");
        let (base_edges, summary) = compute_canonical_edges_from_name(&dynamic_tile_name);

        // Tính mảng cạnh xoay theo Rotation được rút
        let mut rotated_edges = [TerrainType::Empty; 6];
        for i in 0..6 {
            rotated_edges[i] = base_edges[(i + rot) % 6];
        }
        let rotated_str = rotated_edges
            .iter()
            .map(|e| e.to_code())
            .collect::<Vec<_>>()
            .join(" ");

        println!(
            "  • BaseTile #{:02} | Seed: {:11} | Preset: {:14} | Ô Cụ Thể: {:16} | Tóm Tắt: {:12} | Rot Rút: {}",
            n + 1,
            base_seed,
            selected_preset.name,
            dynamic_tile_name,
            summary,
            rot
        );
        println!("     └─► Cạnh Xoay Thực Tế Tại Bàn Cờ (Rotation {}): [{}]", rot, rotated_str);
    }
    println!("-----------------------------------------------------------------------------------------------------------------------\n");
}

fn extract_seg_type_name(val: &serde_json::Value) -> String {
    if let Some(obj) = val.as_object() {
        if let Some(path_id) = obj.get("m_PathID").and_then(|id| id.as_i64()) {
            return match path_id {
                41523 => "1A".to_string(),
                41524 => "2A".to_string(),
                41527 => "2B".to_string(),
                41528 => "2C".to_string(),
                41529 => "3A".to_string(),
                41530 => "3B".to_string(),
                41531 => "3C".to_string(),
                41532 => "3D".to_string(),
                41533 => "4A".to_string(),
                41534 => "4B".to_string(),
                41535 => "4C".to_string(),
                41525 => "5A".to_string(),
                41526 => "6A".to_string(),
                _ => "1A".to_string(),
            };
        }
    }
    "1A".to_string()
}

fn extract_group_type_code(gt_val: &serde_json::Value, name: &str) -> &'static str {
    if let Some(obj) = gt_val.as_object() {
        if let Some(id) = obj.get("m_PathID").and_then(|i| i.as_i64()) {
            match id {
                41478 => return "F",
                41479 => return "A",
                41480 => return "T",
                41481 => return "V",
                41482 => return "W",
                _ => {}
            }
        }
    }
    match name.to_lowercase().as_str() {
        s if s.contains("village") => "V",
        s if s.contains("forest") => "F",
        s if s.contains("agri") => "A",
        s if s.contains("water") => "W",
        s if s.contains("train") => "T",
        _ => "_",
    }
}
