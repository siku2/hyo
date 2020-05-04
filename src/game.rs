use crate::components::{
    card::CardInfo,
    hand::Hand,
    pile::{HiddenPile, VisiblePile},
};
use yew::prelude::*;

type Cards = Vec<CardInfo>;

struct GameState {
    draw_pile: Cards,
    play_pile: Cards,
    holding: Cards,
}

impl GameState {
    fn test() -> Self {
        let cards = vec![
            CardInfo {
                color: String::from("red"),
                number: 8,
            },
            CardInfo {
                color: String::from("red"),
                number: 5,
            },
            CardInfo {
                color: String::from("red"),
                number: 7,
            },
            CardInfo {
                color: String::from("blue"),
                number: 4,
            },
            CardInfo {
                color: String::from("yellow"),
                number: 1,
            },
            CardInfo {
                color: String::from("green"),
                number: 0,
            },
            CardInfo {
                color: String::from("green"),
                number: 2,
            },
        ];

        Self {
            draw_pile: cards.clone(),
            play_pile: vec![CardInfo {
                color: String::from("green"),
                number: 2,
            }],
            holding: cards,
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct GameProps {}

pub struct Game {
    props: GameProps,
    link: ComponentLink<Self>,
    state: GameState,
}

pub enum GameMsg {
    HandCardClicked(usize),
    DrawPileClicked,
}

impl Component for Game {
    type Message = GameMsg;
    type Properties = GameProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = GameState::test();
        Self { props, link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GameMsg::HandCardClicked(i) => {
                let state = &mut self.state;
                if i < state.holding.len() {
                    let card = state.holding.remove(i);
                    state.play_pile.push(card);
                    true
                } else {
                    false
                }
            }
            GameMsg::DrawPileClicked => {
                let state = &mut self.state;
                if let Some(card) = state.draw_pile.pop() {
                    state.holding.push(card);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let state = &self.state;

        let draw_pile_callback = self.link.callback(|_| GameMsg::DrawPileClicked);
        let hand_callback = self.link.callback(|i| GameMsg::HandCardClicked(i));

        html! {
            <div class="table">
                <Hand cards=state.holding.clone() onclick_card=hand_callback/>
                <VisiblePile cards=state.play_pile.clone()/>
                <HiddenPile cards=state.draw_pile.len() onclick=draw_pile_callback/>
            </div>
        }
    }
}
