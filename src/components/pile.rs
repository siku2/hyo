use super::{
    card::{Card, CardProps},
    layout::circle::Circle,
};
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct PileProps {
    pub cards: Vec<CardProps>,
}

pub struct Pile {
    props: PileProps,
}

impl Component for Pile {
    type Message = ();
    type Properties = PileProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let props = &self.props;

        let cards = &props.cards;

        let cards_it =
            cards
                .iter()
                .rev()
                .take(3)
                .rev()
                .cloned()
                .enumerate()
                .map(|(i, mut props)| {
                    let style = format!("--child-index:{};", i);
                    props.angle = std::f32::consts::PI;
                    html! {
                        <div style=style>
                            <Card with props/>
                        </div>
                    }
                });

        html! {
            <div class="pile">
                { for cards_it }
            </div>
        }
    }
}
