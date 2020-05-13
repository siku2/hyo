use uuid::Uuid;

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
}

impl Color {
    const ALL: &'static [Self] = &[Self::Blue, Self::Green, Self::Red, Self::Yellow];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Card {
    Numeric(Color, u8),
    Skip(Color),
    Reverse(Color),
    DrawTwo(Color),

    Wild(Option<Color>),
    WildDrawFour(Option<Color>),
}

impl Card {
    pub fn get_color(&self) -> Option<Color> {
        use Card::*;
        match self {
            Numeric(c, _) | Skip(c) | Reverse(c) | DrawTwo(c) => Some(*c),
            Wild(c) | WildDrawFour(c) => *c,
        }
    }

    fn kind_discriminant(&self) -> impl Eq {
        use Card::*;
        match self {
            Numeric(_, n) => (0, *n),
            Skip(_) => (1, 0),
            Reverse(_) => (2, 0),
            DrawTwo(_) => (3, 0),
            Wild(_) => (4, 0),
            WildDrawFour(_) => (5, 0),
        }
    }

    pub fn playable_on(&self, other: &Self) -> bool {
        // colours are the same or self has no colour yet
        match self.get_color() {
            None => return true,
            c => {
                if c == other.get_color() {
                    return true;
                }
            }
        };

        self.kind_discriminant() == other.kind_discriminant()
    }
}

fn build_default_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(108);

    // 4 * (1 + 1) = 8
    for _ in 0..4 {
        deck.push(Card::Wild(None));
        deck.push(Card::WildDrawFour(None));
    }

    for &color in Color::ALL {
        // 4 * 1 = 4
        deck.push(Card::Numeric(color, 0));

        for _ in 0..2 {
            for n in 1..=9 {
                // 4 * 2 * 9 = 72
                deck.push(Card::Numeric(color, n));
            }
            // 4 * 2 * (1 + 1 + 1) = 24
            deck.push(Card::Skip(color));
            deck.push(Card::DrawTwo(color));
            deck.push(Card::Reverse(color));
        }
    }

    // 8 + 4 + 72 + 24 = 108
    deck
}

fn build_dynamic_deck(len: usize) -> Vec<Card> {
    let default_deck = build_default_deck();
    let n = std::cmp::max(default_deck.len(), len);
    default_deck.iter().cycle().take(n).cloned().collect()
}

pub struct Player {
    pub id: Uuid,
    hand: Vec<Card>,
}

impl Player {
    pub fn empty(id: Uuid) -> Self {
        Self {
            id,
            hand: Vec::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) -> usize {
        self.hand.push(card);
        self.hand.len() - 1
    }

    pub fn remove_card(&mut self, index: usize) -> Option<Card> {
        if index >= self.hand.len() {
            return None;
        }
        Some(self.hand.remove(index))
    }
}

pub enum TurnState {
    PlayOrDraw,
    PlayDrawn(usize),
}

pub struct Game {
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
    players: Vec<Player>,
    current_player_index: usize,
    dir_forward: bool,

    turn_state: TurnState,
}

impl Game {
    fn create(player_ids: impl Iterator<Item = Uuid>) -> Self {
        let mut players: Vec<_> = player_ids.map(Player::empty).collect();
        let mut draw_pile = build_dynamic_deck(players.len() * 20);
        draw_pile.shuffle(&mut thread_rng());

        for player in &mut players {
            for _ in 0..=7 {
                let card = draw_pile.pop().unwrap();
                player.add_card(card);
            }
        }

        let discard_pile = vec![draw_pile.pop().unwrap()];

        Self {
            draw_pile,
            discard_pile,
            players,
            current_player_index: 0,
            dir_forward: true,
            turn_state: TurnState::PlayOrDraw,
        }
    }

    fn current_card(&self) -> &Card {
        // game already starts with one card on the pile
        &self.discard_pile.last().unwrap()
    }

    fn current_player(&mut self) -> &mut Player {
        self.players.get_mut(self.current_player_index).unwrap()
    }

    fn advance(&mut self) {
        let index = self.current_player_index as isize;
        let offset: isize = if self.dir_forward { 1 } else { -1 };
        self.current_player_index =
            (index + offset).rem_euclid(self.players.len() as isize) as usize;

        self.turn_state = TurnState::PlayOrDraw;
    }

    pub fn play_card(&mut self, index: usize) -> bool {
        if !matches!(self.turn_state, TurnState::PlayOrDraw) {
            return false;
        }

        let current_card = self.current_card().clone();
        let player = self.current_player();
        let card = match player.remove_card(index) {
            Some(v) => v,
            None => return false,
        };

        // TODO side effects
        if card.playable_on(&current_card) {
            self.discard_pile.push(card);
            self.advance();
            true
        } else {
            player.add_card(card);
            false
        }
    }

    pub fn draw_card(&mut self) -> bool {
        if !matches!(self.turn_state, TurnState::PlayOrDraw) {
            return false;
        }

        // TODO handle empty draw pile
        let card = match self.draw_pile.pop() {
            Some(v) => v,
            None => return false,
        };

        let can_play = card.playable_on(self.current_card());

        let player = self.current_player();
        let card_index = player.add_card(card);

        if can_play {
            self.turn_state = TurnState::PlayDrawn(card_index);
        } else {
            self.advance();
        }

        true
    }

    pub fn play_drawn_card(&mut self, play: bool) -> bool {
        let card_index = match self.turn_state {
            TurnState::PlayDrawn(v) => v,
            _ => return false,
        };

        if play {
            let card = self.current_player().remove_card(card_index).unwrap();
            self.discard_pile.push(card);
        }

        self.advance();
        true
    }
}
