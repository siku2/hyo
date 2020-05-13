use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}

pub type GameInfoList = Vec<GameInfo>;
