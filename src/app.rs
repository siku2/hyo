use crate::components::{card::CardProps, hand::Hand, pile::Pile};
use yew::prelude::*;

pub struct App;

impl Component for App {
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
        let handle_click = Callback::from(|event: MouseEvent| {
            log::info!("clicked card {:?}", event);
        });

        let cards = vec![
            CardProps {
                color: String::from("red"),
                number: 8,
                angle: 0.0,
                onclick: handle_click.clone(),
            },
            CardProps {
                color: String::from("red"),
                number: 5,
                angle: 0.0,
                onclick: handle_click.clone(),
            },
            CardProps {
                color: String::from("red"),
                number: 7,
                angle: 0.0,
                onclick: handle_click.clone(),
            },
            CardProps {
                color: String::from("blue"),
                number: 4,
                angle: 0.0,
                onclick: handle_click.clone(),
            },
            CardProps {
                color: String::from("yellow"),
                number: 1,
                angle: 0.0,
                onclick: handle_click.clone(),
            },
            CardProps {
                color: String::from("green"),
                number: 0,
                angle: 0.0,
                onclick: handle_click.clone(),
            },
            CardProps {
                color: String::from("green"),
                number: 2,
                angle: 0.0,
                onclick: handle_click.clone(),
            },
        ];

        html! {
            <>
                <Hand cards=cards.clone()/>
                <Pile cards=cards.clone()/>
            </>
        }
    }
}
