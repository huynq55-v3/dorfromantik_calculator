use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
