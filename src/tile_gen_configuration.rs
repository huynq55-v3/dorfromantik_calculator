use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroupType {
    Agriculture,
    Forest,
    Village,
    TrainTrack,
    Water,
}

impl fmt::Display for GroupType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupType::Agriculture => write!(f, "Agriculture"),
            GroupType::Forest => write!(f, "Forest"),
            GroupType::Village => write!(f, "Village"),
            GroupType::TrainTrack => write!(f, "TrainTrack"),
            GroupType::Water => write!(f, "Water"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupTypeConfiguration {
    pub group_type: GroupType,
    pub raw_probability: f32,
}

impl GroupTypeConfiguration {
    pub fn new(group_type: GroupType, raw_probability: f32) -> Self {
        Self {
            group_type,
            raw_probability,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TileGenConfiguration {
    pub global_group_type_probabilities: Vec<GroupTypeConfiguration>,
}

impl TileGenConfiguration {
    pub fn new(global_group_type_probabilities: Vec<GroupTypeConfiguration>) -> Self {
        Self {
            global_group_type_probabilities,
        }
    }

    pub fn get_probability(&self, group_type: GroupType) -> Option<f32> {
        self.global_group_type_probabilities
            .iter()
            .find(|g| g.group_type == group_type)
            .map(|g| g.raw_probability)
    }
}
