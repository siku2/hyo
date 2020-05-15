use crate::{api::SharedAPI, components::game::GamesList, locale::Locale};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct GamesViewProps {
    pub api: SharedAPI,
    pub locale: Locale,
}
pub struct GamesView {
    props: GamesViewProps,
}

impl Component for GamesView {
    type Message = ();
    type Properties = GamesViewProps;

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
            <div class="games-layout">
                <div class="games-layout__background"/>

                <div class="games-layout__header">
                    <button>{ "BACK" }</button>
                </div>

                <GamesList api=props.api.clone()/>
            </div>
        }
    }
}
