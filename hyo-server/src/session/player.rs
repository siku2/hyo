use super::connection::Connection;
use uuid::Uuid;

pub struct Player {
    pub id: Uuid,
    pub conn: Connection,
}
