use crate::hex::AxialPos;
use crate::tile::SegmentType;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct FeatureNode {
    pub pos: AxialPos,
    pub segment_type: SegmentType,
    pub edge_idx: usize,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub parent: usize,
    pub segment_type: SegmentType,
    pub tiles: HashSet<AxialPos>,
    pub open_edges_count: usize,
}

pub struct GroupManager {
    groups: Vec<Group>,
    // Maps (pos, segment_type) -> group index
    node_to_group: HashMap<(AxialPos, SegmentType), usize>,
}

impl GroupManager {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            node_to_group: HashMap::new(),
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.groups[i].parent == i {
            i
        } else {
            let p = self.groups[i].parent;
            let root = self.find(p);
            self.groups[i].parent = root;
            root
        }
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.groups[root_j].parent = root_i;
            let j_tiles = std::mem::take(&mut self.groups[root_j].tiles);
            self.groups[root_i].tiles.extend(j_tiles);
            self.groups[root_i].open_edges_count += self.groups[root_j].open_edges_count;
        }
    }

    pub fn add_tile_segments(
        &mut self,
        pos: AxialPos,
        edges: &[SegmentType; 6],
        adjacent_placed: &HashMap<AxialPos, [SegmentType; 6]>,
    ) {
        // Group edges on this tile by segment type
        let mut type_edges: HashMap<SegmentType, Vec<usize>> = HashMap::new();
        for (dir, &stype) in edges.iter().enumerate() {
            type_edges.entry(stype).or_default().push(dir);
        }

        for (stype, dir_list) in type_edges {
            let group_idx = self.groups.len();
            let mut tiles = HashSet::new();
            tiles.insert(pos);

            let open_edges = dir_list.len();

            self.groups.push(Group {
                parent: group_idx,
                segment_type: stype,
                tiles,
                open_edges_count: open_edges,
            });

            self.node_to_group.insert((pos, stype), group_idx);

            // Connect with neighbors of the same segment type
            for &dir in &dir_list {
                let neighbor_pos = pos.neighbor(dir);
                if let Some(neighbor_edges) = adjacent_placed.get(&neighbor_pos) {
                    let opp_dir = AxialPos::opposite_dir(dir);
                    if neighbor_edges[opp_dir] == stype {
                        if let Some(&neighbor_group) = self.node_to_group.get(&(neighbor_pos, stype)) {
                            let root_curr = self.find(group_idx);
                            let root_neigh = self.find(neighbor_group);
                            if root_curr != root_neigh {
                                self.union(root_curr, root_neigh);
                            }
                            // Subtract 2 open edges (one from each tile)
                            let root = self.find(group_idx);
                            if self.groups[root].open_edges_count >= 2 {
                                self.groups[root].open_edges_count -= 2;
                            } else {
                                self.groups[root].open_edges_count = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_group_size(&mut self, pos: AxialPos, stype: SegmentType) -> usize {
        if let Some(&g) = self.node_to_group.get(&(pos, stype)) {
            let root = self.find(g);
            self.groups[root].tiles.len()
        } else {
            0
        }
    }

    pub fn is_group_closed(&mut self, pos: AxialPos, stype: SegmentType) -> bool {
        if let Some(&g) = self.node_to_group.get(&(pos, stype)) {
            let root = self.find(g);
            self.groups[root].open_edges_count == 0
        } else {
            false
        }
    }
}
