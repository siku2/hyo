use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct CardProps {
    pub color: String,
    pub number: u8,
}

pub struct Card {
    props: CardProps,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

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
        let class_str = format!("card card-{}", props.color);

        html! {
            <div class={class_str}>
                <div class="card-number top-left">{props.number}</div>
                <div class="card-number center">{props.number}</div>
                <div class="card-number bottom-right">{props.number}</div>
            </div>
        }
    }
}
