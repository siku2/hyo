use std::collections::HashMap;
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

#[derive(Debug, Default)]
pub struct SessionManager {
    sessions: HashMap<Uuid, Session>,
}

impl SessionManager {
    pub fn iter_public_sessions(&self) -> impl Iterator<Item = &Session> {
        self.sessions.values().filter(|sess| sess.settings.public)
    }

    fn new_session_id(&self) -> Uuid {
        loop {
            let id = Uuid::new_v4();
            if !self.sessions.contains_key(&id) {
                return id;
            }
        }
    }
}
