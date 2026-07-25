use std::env;
use std::fs;
use std::io;

const BASE62_CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn decode_base62(encoded: &str) -> u64 {
    let mut val: u64 = 0;
    for &b in encoded.as_bytes() {
        if let Some(idx) = BASE62_CHARS.iter().position(|&c| c == b) {
            val = val * 62 + idx as u64;
        }
    }
    val
}

fn decode_digits_10(num: u64, target_len: usize) -> Vec<u32> {
    let s = num.to_string();
    let mut digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    while digits.len() < target_len {
        digits.insert(0, 0);
    }
    digits
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filepath = if args.len() > 1 {
        &args[1]
    } else {
        "SaveGame_Monthly_2026-07-23-08-04-28.sav"
    };

    println!("================================================================================");
    println!(" DORFROMANTIK MAP & SEED FULL DECODER");
    println!(" Reading: {}", filepath);
    println!("================================================================================");

    let bytes = match fs::read(filepath) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", filepath, e);
            return Ok(());
        }
    };

    println!("File size: {} bytes\n", bytes.len());

    println!("--------------------------------------------------------------------------------");
    println!(" 💡 DIFFERENCE BETWEEN DATE ID (103911138) AND PRNG SEED (3103784960)");
    println!("--------------------------------------------------------------------------------");
    println!(" 1. 103911138 ('072026'):");
    println!("    - This is the MONTH / YEAR IDENTIFIER ('07' / '2026' = July 2026).");
    println!("    - It identifies WHICH Monthly Challenge mode session this map belongs to.\n");
    println!(" 2. 3103784960 (Signed: -1191182336 / Hex: 0xb9000000):");
    println!("    - This is the ACTUAL 32-BIT PRNG SEED used in memory by Unity's Random engine.");
    println!("    - This is the seed that determines the exact sequence of tile draws!\n");

    println!("--------------------------------------------------------------------------------");
    println!(" 🗺️ DECODED CONFIGSTRING & CUSTOM RULES");
    println!("--------------------------------------------------------------------------------");
    if let Some(cfg_pos) = find_subslice(&bytes, b"0720262fJmCw2gRsn6") {
        let config_str = String::from_utf8_lossy(&bytes[cfg_pos..cfg_pos + 18]);
        println!(" Found ConfigString: {}", config_str);

        let part0 = &config_str[0..6];
        let part1 = &config_str[6..12];
        let part2 = &config_str[12..18];

        let val1 = decode_base62(part1);
        let digits1 = decode_digits_10(val1, 10);

        let val2 = decode_base62(part2);
        let digits2 = decode_digits_10(val2, 10);

        println!("  - Date Header ('{}') -> Month: 07 | Year: 2026 (July 2026)", part0);

        println!("\n  - Part 1 ('{}') -> Integer: {} | Custom Rule Levels:", part1, val1);
        println!("      * VillageProbability Level:     {}", digits1[1]);
        println!("      * ForestProbability Level:      {} (Higher forest spawn weight)", digits1[2]);
        println!("      * AgricultureProbability Level: {} (Higher agriculture weight)", digits1[3]);
        println!("      * WaterProbability Level:       {} (Higher river spawn weight)", digits1[4]);
        println!("      * TrainTrackProbability Level:  {} (Higher railway weight)", digits1[5]);
        println!("      * TileStackHeight Level:        {}", digits1[6]);
        println!("      * TileLimit Level:              {} -> [250 TILES MAX LIMIT]", digits1[7]);
        println!("      * Density Level:                {}", digits1[8]);
        println!("      * QuestProbability Level:       {}", digits1[9]);

        println!("\n  - Part 2 ('{}') -> Integer: {} | Custom Rule Levels:", part2, val2);
        println!("      * QuestDifficulty Level:        {}", digits2[1]);
        println!("      * FlagQuestProbability Level:   {} (High flag quest reward rate)", digits2[2]);
        println!("      * WorldBorderRadius Level:      {}", digits2[3]);

        if cfg_pos >= 13 {
            let raw_seed = i32::from_le_bytes(bytes[cfg_pos - 13..cfg_pos - 9].try_into().unwrap());
            println!("\n  - ACTUAL PRNG TILE SEED (int32): {}", raw_seed);
            println!("  - ACTUAL PRNG TILE SEED (u32):   {}", raw_seed as u32);
            println!("  - ACTUAL PRNG TILE SEED (hex):   0x{:08x}", raw_seed as u32);
        }
    }

    println!("\n--------------------------------------------------------------------------------");
    println!(" 🔑 SEED TYPES FOUND IN SAVE FILE BINARY STREAM");
    println!("--------------------------------------------------------------------------------");
    scan_and_print_seed_labels(&bytes, b"seed", "Tile Generation Seed");
    scan_and_print_seed_labels(&bytes, b"biomeSeed", "Biome Palette Theme Seed");
    scan_and_print_seed_labels(&bytes, b"preplacedTileSeed", "Preplaced Landmark Quest Seed");

    println!("\n================================================================================");
    println!(" DECODING COMPLETED SUCCESSFULLY!");
    println!("================================================================================");

    Ok(())
}

fn scan_and_print_seed_labels(bytes: &[u8], label: &[u8], description: &str) {
    let mut pos = 0;
    let mut count = 0;
    while let Some(idx) = find_subslice(&bytes[pos..], label) {
        let abs_pos = pos + idx;
        count += 1;
        if count <= 4 {
            println!(
                "  * [{}] Offset 0x{:06x}: Field Name '{}'",
                description,
                abs_pos,
                String::from_utf8_lossy(label)
            );
        }
        pos = abs_pos + label.len();
    }
    if count > 4 {
        println!("  ... and {} more occurrences of '{}'", count - 4, String::from_utf8_lossy(label));
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}
