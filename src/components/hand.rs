use super::{
    card::{CardFront, CardInfo},
    layout::circle::Circle,
};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct HandProps {
    pub cards: Vec<CardInfo>,
}

pub struct Hand {
    props: HandProps,
}

impl Component for Hand {
    type Message = ();
    type Properties = HandProps;

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
        let cards: Vec<_> = self
            .props
            .cards
            .iter()
            .cloned()
            .map(|info| {
                html! {
                    <CardFront color=info.color number=info.number/>
                }
            })
            .collect();

        html! {
            <div class="hand">
                <Circle target_angle=22.5 max_total_angle=180.0>
                    { cards }
                </Circle>
            </div>
        }
    }
}
