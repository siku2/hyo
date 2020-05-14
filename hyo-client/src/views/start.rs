use super::GamesView;
use crate::{
    api::{SharedAPI, API},
    components::icon::MDIcon,
    locale::Locale,
};
use reqwest::Url;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct StartViewProps {
    pub locale: Locale,
}
pub struct StartView {
    props: StartViewProps,
    api: SharedAPI,
}

impl Component for StartView {
    type Message = ();
    type Properties = StartViewProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            api: SharedAPI::from(API::new(Url::parse("http://localhost:8000/").unwrap())),
        }
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
                <GamesView api=self.api.clone() locale=locale.clone()/>

                <div class="start-layout__background"/>

                <h1 class="start-layout__title">{ locale.localize("title", None) }</h1>
                <div class="start-layout__buttons button-row button-row--center">
                    <button class="button-row__btn">{ locale.localize("create-game", None) }</button>
                    <button class="button-row__btn">{ locale.localize("join-game", None) }</button>
                </div>
                <div class="start-layout__settings">
                    <MDIcon icon="settings"/>
                </div>
            </div>
        }
    }
}
