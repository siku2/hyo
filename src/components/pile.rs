use super::card::{CardBack, CardFront, CardInfo};
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct VisiblePileProps {
    pub cards: Vec<CardInfo>,
}

pub struct VisiblePile {
    props: VisiblePileProps,
}

impl Component for VisiblePile {
    type Message = ();
    type Properties = VisiblePileProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        let props = &self.props;

        let cards = &props.cards;

        let cards_it = cards
            .iter()
            .rev()
            .cloned()
            .enumerate()
            .map(|(i, info)| {
                html! {
                    <div style=format!("--child-index:{};", i)>
                        <CardFront color=info.color number=info.number/>
                    </div>
                }
            })
            .take(3)
            .rev();

        html! {
            <div class="pile">
                { for cards_it }
            </div>
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct HiddenPileProps {
    pub cards: usize,
}

pub struct HiddenPile {
    props: HiddenPileProps,
}

impl Component for HiddenPile {
    type Message = ();
    type Properties = HiddenPileProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        let props = &self.props;

        html! {
            <div class="pile">
                <CardBack/>
            </div>
        }
    }
}
