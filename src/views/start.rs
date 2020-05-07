use crate::locale::Locale;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct StartViewProps {
    pub locale: Locale,
}
pub struct StartView {
    props: StartViewProps,
}

impl Component for StartView {
    type Message = ();
    type Properties = StartViewProps;

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
        let locale = &props.locale;

        html! {
            <div class="start-layout">
                <h1 class="start-layout__title">{ locale.localize("title", None) }</h1>
                <div class="start-layout__buttons">
                    <a>{ locale.localize("create-game", None) }</a>
                    <a>{ locale.localize("join-game", None) }</a>
                </div>
            </div>
        }
    }
}
