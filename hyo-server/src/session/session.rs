use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionSettings {
    pub public: bool,
    pub max_players: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
    pub id: Uuid,
    pub game_id: String,
    pub settings: SessionSettings,
}
