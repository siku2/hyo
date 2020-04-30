use crate::components::{card::CardInfo, hand::Hand, pile::{VisiblePile, HiddenPile}};
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

        html! {
            <>
                <Hand cards=cards.clone()/>
                <div class="table">
                    <VisiblePile cards=cards.clone()/>
                    <HiddenPile cards=13/>
                </div>
            </>
        }
    }
}
