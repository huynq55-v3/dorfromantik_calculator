#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SegmentType {
    Grass,
    Village,
    Forest,
    Agriculture,
    Water,
    Train,
}

impl SegmentType {
    pub fn name(&self) -> &'static str {
        match self {
            SegmentType::Grass => "Grass",
            SegmentType::Village => "Village",
            SegmentType::Forest => "Forest",
            SegmentType::Agriculture => "Agriculture",
            SegmentType::Water => "Water",
            SegmentType::Train => "Train",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestType {
    MoreThan,
    Exactly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quest {
    pub target_type: SegmentType,
    pub quest_type: QuestType,
    pub target_count: usize,
    pub is_fulfilled: bool,
    pub is_flag: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub edges: [SegmentType; 6],
    pub rotation: usize, // 0..5 (multiples of 60 deg)
    pub quest: Option<Quest>,
}

impl Tile {
    pub fn new(edges: [SegmentType; 6], quest: Option<Quest>) -> Self {
        Self {
            edges,
            rotation: 0,
            quest,
        }
    }

    /// Rotate tile 60 degrees clockwise.
    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 6;
        let mut new_edges = [SegmentType::Grass; 6];
        for i in 0..6 {
            new_edges[(i + 1) % 6] = self.edges[i];
        }
        self.edges = new_edges;
    }

    pub fn get_edge(&self, dir: usize) -> SegmentType {
        self.edges[dir % 6]
    }
}
