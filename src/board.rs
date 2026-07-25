use crate::group::GroupManager;
use crate::hex::AxialPos;
use crate::tile::{QuestType, SegmentType, Tile};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PlacedTile {
    pub tile: Tile,
    pub is_perfect: bool,
}

pub struct Board {
    pub tiles: HashMap<AxialPos, PlacedTile>,
    pub valid_slots: HashSet<AxialPos>,
    pub group_manager: GroupManager,
}

pub struct PlacementResult {
    pub points_awarded: u32,
    pub tiles_added_to_deck: u32,
    pub is_perfect: bool,
    pub quests_completed: u32,
    pub flags_completed: u32,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            tiles: HashMap::new(),
            valid_slots: HashSet::new(),
            group_manager: GroupManager::new(),
        };

        // Center position (0, 0) is valid initially
        board.valid_slots.insert(AxialPos::ZERO);
        board
    }

    pub fn can_place(&self, pos: AxialPos) -> bool {
        if self.tiles.is_empty() {
            pos == AxialPos::ZERO
        } else {
            !self.tiles.contains_key(&pos) && self.valid_slots.contains(&pos)
        }
    }

    pub fn place_tile(&mut self, pos: AxialPos, tile: Tile) -> Option<PlacementResult> {
        if !self.can_place(pos) {
            return None;
        }

        let mut points = 0;
        let mut extra_tiles = 0;
        let mut quests_completed = 0;
        let mut flags_completed = 0;

        // 1. Calculate Edge Matching Score (+10 per matching edge)
        let mut matching_edges = 0;
        let mut total_adjacent_neighbors = 0;

        for dir in 0..6 {
            let neighbor_pos = pos.neighbor(dir);
            if let Some(neighbor_tile) = self.tiles.get(&neighbor_pos) {
                total_adjacent_neighbors += 1;
                let my_edge = tile.get_edge(dir);
                let opp_dir = AxialPos::opposite_dir(dir);
                let neighbor_edge = neighbor_tile.tile.get_edge(opp_dir);

                if my_edge == neighbor_edge {
                    matching_edges += 1;
                    points += 10;
                }
            }
        }

        // Perfect match: All 6 edges match adjacent placed neighbors or closed boundaries
        let is_perfect = total_adjacent_neighbors == 6 && matching_edges == 6;
        if is_perfect {
            points += 60;
            extra_tiles += 1; // +1 Tile for Perfect Placement
        }

        // 2. Add to placed tiles map & Update valid slots
        let placed = PlacedTile {
            tile: tile.clone(),
            is_perfect,
        };
        self.tiles.insert(pos, placed);
        self.valid_slots.remove(&pos);

        // Add adjacent empty hexes to valid_slots
        for dir in 0..6 {
            let npos = pos.neighbor(dir);
            if !self.tiles.contains_key(&npos) {
                self.valid_slots.insert(npos);
            }
        }

        // 3. Update Union-Find Group Manager
        let adjacent_map: HashMap<AxialPos, [SegmentType; 6]> = self
            .tiles
            .iter()
            .map(|(&p, pt)| (p, pt.tile.edges))
            .collect();

        self.group_manager
            .add_tile_segments(pos, &tile.edges, &adjacent_map);

        // 4. Evaluate Quests across all tiles
        let mut quest_updates = Vec::new();

        for (&tpos, pt) in self.tiles.iter() {
            if let Some(ref q) = pt.tile.quest {
                if !q.is_fulfilled {
                    let group_size = self.group_manager.get_group_size(tpos, q.target_type);
                    let fulfilled = match q.quest_type {
                        QuestType::MoreThan => group_size >= q.target_count,
                        QuestType::Exactly => group_size == q.target_count,
                    };

                    if fulfilled {
                        quest_updates.push((tpos, q.is_flag));
                    }
                }
            }
        }

        for (tpos, is_flag) in quest_updates {
            if let Some(pt) = self.tiles.get_mut(&tpos) {
                if let Some(ref mut q) = pt.tile.quest {
                    q.is_fulfilled = true;
                    points += 100;
                    extra_tiles += 5; // +5 Tiles for Quest / Flag Completion

                    if is_flag {
                        flags_completed += 1;
                    } else {
                        quests_completed += 1;
                        // Turn into Flag quest on completion
                        q.is_flag = true;
                        q.is_fulfilled = false; // Now tracks closing
                    }
                }
            }
        }

        // 5. Evaluate Flag Quests (Closing groups)
        let mut flag_updates = Vec::new();
        for (&tpos, pt) in self.tiles.iter() {
            if let Some(ref q) = pt.tile.quest {
                if q.is_flag && !q.is_fulfilled {
                    if self.group_manager.is_group_closed(tpos, q.target_type) {
                        flag_updates.push(tpos);
                    }
                }
            }
        }

        for tpos in flag_updates {
            if let Some(pt) = self.tiles.get_mut(&tpos) {
                if let Some(ref mut q) = pt.tile.quest {
                    q.is_fulfilled = true;
                    points += 100;
                    extra_tiles += 5; // +5 Tiles for Flag completion
                    flags_completed += 1;
                }
            }
        }

        Some(PlacementResult {
            points_awarded: points,
            tiles_added_to_deck: extra_tiles,
            is_perfect,
            quests_completed,
            flags_completed,
        })
    }
}
