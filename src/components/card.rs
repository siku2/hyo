use yew::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct CardFrontProps {
    pub color: String,
    pub number: u8,
}
pub struct CardFront {
    props: CardFrontProps,
}

impl Component for CardFront {
    type Message = ();
    type Properties = CardFrontProps;

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
        let style = format!("--card-color: {}", props.color);

        html! {
            <div class="card-front" style=style>
                <div class="card__number top-left">{props.number}</div>
                <div class="card__number center">{props.number}</div>
                <div class="card__number bottom-right">{props.number}</div>
            </div>
        }
    }
}

pub struct CardBack {}

impl Component for CardBack {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="card-back"/>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    pub color: String,
    pub number: u8,

    #[prop_or(0.0)]
    pub angle: f32,

    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
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
            <div class="card-rotate__container" onclick=props.onclick.clone()>
                <div class="card-rotate" style=format!("transform: rotateY({}rad)", props.angle)>
                    <CardFront color=props.color.clone() number=props.number/>
                    <CardBack/>
                </div>
            </div>
        }
    }
}
