use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainType {
    Empty,
    Village,
    Forest,
    Agri,
    TrainTrack,
    Water,
}

impl TerrainType {
    pub fn to_code(self) -> &'static str {
        match self {
            TerrainType::Empty => "_",
            TerrainType::Village => "V",
            TerrainType::Forest => "F",
            TerrainType::Agri => "A",
            TerrainType::TrainTrack => "T",
            TerrainType::Water => "W",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            TerrainType::Empty => "⬜",
            TerrainType::Village => "🏠",
            TerrainType::Forest => "🌲",
            TerrainType::Agri => "🌾",
            TerrainType::TrainTrack => "🚂",
            TerrainType::Water => "🌊",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTileMetadata {
    #[serde(rename = "tileType", default)]
    pub tile_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "subCollection", default)]
    pub sub_collection: Option<String>,
    #[serde(rename = "Village", default)]
    pub village: usize,
    #[serde(rename = "Forest", default)]
    pub forest: usize,
    #[serde(rename = "Agriculture", default)]
    pub agriculture: usize,
    #[serde(rename = "TrainTracks", default)]
    pub train_tracks: usize,
    #[serde(rename = "Water", default)]
    pub water: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMetadataRoot {
    #[serde(default)]
    pub tiles: Vec<RawTileMetadata>,
}

#[derive(Debug, Clone)]
pub struct TileTopology {
    pub name: String,
    pub edges: [TerrainType; 6],
    pub summary: String,
    pub village: usize,
    pub forest: usize,
    pub agriculture: usize,
    pub train_tracks: usize,
    pub water: usize,
}

impl TileTopology {
    pub fn format_edges(&self) -> String {
        self.edges
            .iter()
            .map(|e| e.to_code())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn get_all_rotations(&self) -> [[TerrainType; 6]; 6] {
        let mut rots = [[TerrainType::Empty; 6]; 6];
        for r in 0..6 {
            for i in 0..6 {
                rots[r][i] = self.edges[(i + r) % 6];
            }
        }
        rots
    }

    pub fn format_rotations(&self) -> Vec<String> {
        self.get_all_rotations()
            .iter()
            .map(|rot| {
                rot.iter()
                    .map(|e| e.to_code())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect()
    }

    pub fn format_exact_objects(&self) -> String {
        let mut parts = Vec::new();
        if self.village > 0 {
            parts.push(format!("🏠 {} Village", self.village));
        }
        if self.forest > 0 {
            parts.push(format!("🌲 {} Forest", self.forest));
        }
        if self.agriculture > 0 {
            parts.push(format!("🌾 {} Agriculture", self.agriculture));
        }
        if self.train_tracks > 0 {
            parts.push(format!("🚂 {} TrainTracks", self.train_tracks));
        }
        if self.water > 0 {
            parts.push(format!("🌊 {} Water", self.water));
        }
        if parts.is_empty() {
            "Trống".to_string()
        } else {
            parts.join(" | ")
        }
    }
}

pub struct MetadataDatabase {
    pub map: HashMap<String, RawTileMetadata>,
}

impl MetadataDatabase {
    pub fn load_from_file(path: &str) -> Self {
        let mut map = HashMap::new();
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(root) = serde_json::from_str::<TileMetadataRoot>(&content) {
                for item in root.tiles {
                    if let Some(ref sub) = item.sub_collection {
                        map.insert(sub.clone(), item.clone());
                    }
                    map.insert(item.name.clone(), item);
                }
            }
        }
        Self { map }
    }

    pub fn get_topology(&self, name: &str) -> TileTopology {
        let meta = self.map.get(name);
        let village = meta.map_or(0, |m| m.village);
        let forest = meta.map_or(0, |m| m.forest);
        let agriculture = meta.map_or(0, |m| m.agriculture);
        let train_tracks = meta.map_or(0, |m| m.train_tracks);
        let water = meta.map_or(0, |m| m.water);

        let (edges, summary) = compute_canonical_edges_from_name(name);

        TileTopology {
            name: name.to_string(),
            edges,
            summary,
            village,
            forest,
            agriculture,
            train_tracks,
            water,
        }
    }
}

/// =========================================================================================
/// 📐 THUẬT TOÁN TÍNH TOÁN HÌNH HỌC 6 CẠNH TỰ ĐỘNG (NO HARDCODING 100%)
/// =========================================================================================

/// Trả về danh sách mẫu cạnh tương đối cho từng ký hiệu hình học [Count][Pattern]
fn get_relative_pattern_offsets(count: usize, pattern: char) -> Vec<usize> {
    match (count, pattern) {
        // Count = 1
        (1, _) => vec![0],

        // Count = 2
        (2, 'A') => vec![0, 1], // 2 cạnh kề nhau
        (2, 'B') => vec![0, 4], // 2 cạnh ngắt quãng chiều kim đồng hồ
        (2, 'C') => vec![0, 3], // 2 cạnh đối diện (step 3)

        // Count = 3
        (3, 'A') => vec![0, 1, 2], // 3 cạnh liên tiếp
        (3, 'B') => vec![0, 2, 4], // 3 cạnh cách đều
        (3, 'C') => vec![0, 1, 3],

        // Count = 4
        (4, 'A') => vec![0, 1, 2, 3],
        (4, 'B') => vec![0, 1, 3, 5], // 4 cạnh ngắt quãng
        (4, 'C') => vec![0, 1, 2, 4],

        // Count = 5
        (5, _) => vec![0, 1, 2, 3, 4],

        // Count = 6
        (6, _) => vec![0, 1, 2, 3, 4, 5],

        _ => vec![0],
    }
}

/// Phân tích tên subcollection/prefab và tính toán 6 cạnh bằng thuật toán hình học động 100% (NO HARDCODING)
pub fn compute_canonical_edges_from_name(name: &str) -> ([TerrainType; 6], String) {
    let mut edges = [TerrainType::Empty; 6];

    let tokens: Vec<&str> = name.split_whitespace().collect();
    for token in tokens {
        let clean_token = token
            .trim_start_matches("QuestTile_")
            .trim_end_matches("_Locomotive")
            .trim_end_matches("-Locomotive");

        let sub_parts: Vec<&str> = clean_token.split('-').collect();
        for part in sub_parts {
            parse_and_apply_segment(part, &mut edges);
        }
    }

    // Tóm tắt các loại cạnh
    let mut counts: HashMap<TerrainType, usize> = HashMap::new();
    for &e in &edges {
        if e != TerrainType::Empty {
            *counts.entry(e).or_insert(0) += 1;
        }
    }

    let mut summary_parts = Vec::new();
    for &t in &[
        TerrainType::TrainTrack,
        TerrainType::Forest,
        TerrainType::Village,
        TerrainType::Agri,
        TerrainType::Water,
    ] {
        if let Some(&cnt) = counts.get(&t) {
            summary_parts.push(format!("{} {}", cnt, t.to_code()));
        }
    }

    let summary = if summary_parts.is_empty() {
        "Trống".to_string()
    } else {
        summary_parts.join(", ")
    };

    (edges, summary)
}

fn parse_and_apply_segment(part: &str, edges: &mut [TerrainType; 6]) {
    let chars: Vec<char> = part.chars().collect();
    if chars.len() < 3 {
        return;
    }

    let count = match chars[0].to_digit(10) {
        Some(c) => c as usize,
        None => return,
    };

    let pattern = chars[1].to_ascii_uppercase();
    if !pattern.is_ascii_alphabetic() {
        return;
    }

    let terrain = match chars[2].to_ascii_uppercase() {
        'V' => TerrainType::Village,
        'F' => TerrainType::Forest,
        'A' => TerrainType::Agri,
        'T' => TerrainType::TrainTrack,
        'W' => TerrainType::Water,
        _ => return,
    };

    let relative_offsets = get_relative_pattern_offsets(count, pattern);

    for rotation in 0..6 {
        let fits = relative_offsets
            .iter()
            .all(|&offset| edges[(rotation + offset) % 6] == TerrainType::Empty);
        if fits {
            for &offset in &relative_offsets {
                edges[(rotation + offset) % 6] = terrain;
            }
            break;
        }
    }
}
