use super::{
    card::{CardFront, CardInfo},
    layout::circle::Circle,
};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct HandProps {
    pub cards: Vec<CardInfo>,

    #[prop_or_else(Callback::noop)]
    pub onclick_card: Callback<usize>,
}

pub struct Hand {
    props: HandProps,
    link: ComponentLink<Self>,
}

pub enum HandMsg {
    CardClick(usize),
}

impl Component for Hand {
    type Message = HandMsg;
    type Properties = HandProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let props = &self.props;

        let HandMsg::CardClick(i) = msg;
        props.onclick_card.emit(i);

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
            .enumerate()
            .map(|(i, info)| {
                let callback = self.link.callback(move |_| HandMsg::CardClick(i));

                html! {
                    <CardFront color=info.color number=info.number onclick=callback/>
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
