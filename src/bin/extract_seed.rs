use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filepath = if args.len() > 1 {
        &args[1]
    } else {
        "SaveGame_Monthly_2026-07-23-08-04-28.sav"
    };

    println!("==================================================");
    println!(" DORFROMANTIK SAVE FILE SEED EXTRACTOR");
    println!(" Reading: {}", filepath);
    println!("==================================================");

    let bytes = match fs::read(filepath) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", filepath, e);
            return Ok(());
        }
    };

    println!("File size: {} bytes\n", bytes.len());

    // 1. Scan for CustomModeData Seed & Config String
    println!("--- 1. Main Tile Generator Seed (CustomModeData) ---");
    if let Some(cfg_pos) = find_subslice(&bytes, b"0720262fJmCw2gRsn6") {
        println!("  Found Monthly ConfigString at offset 0x{:x}", cfg_pos);
        let config_str = String::from_utf8_lossy(&bytes[cfg_pos..cfg_pos + 18]);
        println!("  ConfigString: {}", config_str);

        // Year and Month follow configString
        if cfg_pos + 18 + 8 <= bytes.len() {
            let year = i32::from_le_bytes(bytes[cfg_pos + 18..cfg_pos + 22].try_into().unwrap());
            let month = i32::from_le_bytes(bytes[cfg_pos + 22..cfg_pos + 26].try_into().unwrap());
            println!("  Year: {}", year);
            println!("  Month: {}", month);
        }

        // Seed is written 13 bytes before configString
        if cfg_pos >= 13 {
            let seed_raw = i32::from_le_bytes(bytes[cfg_pos - 13..cfg_pos - 9].try_into().unwrap());
            println!("  Tile Generation Seed (int32): {}", seed_raw);
            println!("  Tile Generation Seed (u32):   {}", seed_raw as u32);
            println!("  Tile Generation Seed (hex):   0x{:08x}", seed_raw as u32);
        }
    } else {
        println!("  Monthly ConfigString pattern not found directly, scanning all 'seed' labels...");
    }

    // 2. Scan all string positions matching "seed", "biomeSeed", "preplacedTileSeed"
    println!("\n--- 2. All Seed Field Occurrences in Binary Stream ---");
    scan_and_print_seed_labels(&bytes, b"seed", "General Seed");
    scan_and_print_seed_labels(&bytes, b"biomeSeed", "Biome Visual Seed");
    scan_and_print_seed_labels(&bytes, b"preplacedTileSeed", "Preplaced Landmark Seed");

    // 3. Summary of Seed Types in Dorfromantik
    println!("\n==================================================");
    println!(" SUMMARY: TYPES OF SEEDS IN DORFROMANTIK");
    println!("==================================================");
    println!(" 1. Tile Generation Seed (tileGenerator.Seed / seed):");
    println!("    - Main 32-bit PRNG seed controlling tile queue and quest generation.");
    println!("    - Used by TileDeckGenerator to produce deterministic games.\n");
    println!(" 2. Biome Seed (biomeSeed):");
    println!("    - Controls visual biome palette theme selection (Standard, Lavender, Fjord, etc.).\n");
    println!(" 3. Preplaced Landmark Seed (preplacedTileSeed):");
    println!("    - Controls spawning of pre-placed quest landmark sites out in fog.\n");
    println!(" 4. Per-Tile Visual Decoration Seed (tile.Seed / seedOffset):");
    println!("    - Attached to individual placed tiles to randomize 3D asset rotations/scales.");
    println!("==================================================");

    Ok(())
}

fn scan_and_print_seed_labels(bytes: &[u8], label: &[u8], description: &str) {
    let mut pos = 0;
    let mut count = 0;
    while let Some(idx) = find_subslice(&bytes[pos..], label) {
        let abs_pos = pos + idx;
        count += 1;
        if count <= 5 {
            println!(
                "  [{}] Offset 0x{:06x}: Label '{}'",
                description,
                abs_pos,
                String::from_utf8_lossy(label)
            );
        }
        pos = abs_pos + label.len();
    }
    if count > 5 {
        println!("  ... and {} more occurrences of '{}'", count - 5, String::from_utf8_lossy(label));
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}
