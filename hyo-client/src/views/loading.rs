use super::StartView;
use crate::locale::{FetchFluentError, Locale};
use yew::prelude::*;

pub struct LoadingView {
    locale: Option<Locale>,
}

pub enum LoadingViewMsg {
    LocaleLoaded(Result<Locale, FetchFluentError>),
}

impl Component for LoadingView {
    type Message = LoadingViewMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        wasm_bindgen_futures::spawn_local(async move {
            let locale = Locale::load_for_user().await;
            link.send_message(LoadingViewMsg::LocaleLoaded(locale));
        });

        Self { locale: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LoadingViewMsg::LocaleLoaded(res) => {
                // TODO error handling
                self.locale = res.ok();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        if let Some(locale) = &self.locale {
            html! {
                <StartView locale=locale.clone()/>
            }
        } else {
            html! {}
        }
    }
}
