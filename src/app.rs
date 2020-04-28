use crate::components::{card::CardProps, hand::Hand};
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
        let cards = vec![
            CardProps {
                color: String::from("red"),
                number: 8,
            },
            CardProps {
                color: String::from("red"),
                number: 5,
            },
            CardProps {
                color: String::from("red"),
                number: 7,
            },
            CardProps {
                color: String::from("blue"),
                number: 4,
            },
            CardProps {
                color: String::from("yellow"),
                number: 1,
            },
            CardProps {
                color: String::from("green"),
                number: 0,
            },
            CardProps {
                color: String::from("green"),
                number: 2,
            },

            CardProps {
                color: String::from("red"),
                number: 8,
            },
            CardProps {
                color: String::from("red"),
                number: 5,
            },
            CardProps {
                color: String::from("red"),
                number: 7,
            },
            CardProps {
                color: String::from("blue"),
                number: 4,
            },
            CardProps {
                color: String::from("yellow"),
                number: 1,
            },
            CardProps {
                color: String::from("green"),
                number: 0,
            },
            CardProps {
                color: String::from("green"),
                number: 2,
            },
        ];

        html! {
            <>
                <Hand cards=cards/>
            </>
        }
    }
}
