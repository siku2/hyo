use super::game::Card;
use std::collections::HashMap;
use uuid::Uuid;

pub struct GameState {
    discard_pile: Vec<Card>,
    draw_pile: usize,
    player_hands: HashMap<Uuid, usize>,
    current_player: Uuid,
    dir_forward: bool,
}

pub struct PlayerState {
    hand: Vec<Card>,
}

pub enum ClientAction {
    PlayCard(usize),
    DrawCard,
}
