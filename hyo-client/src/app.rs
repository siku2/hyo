use crate::{
    api::{SharedAPI, API},
    locale::{FetchFluentError, Locale},
    views::{GamesView, StartView},
};
use std::borrow::Borrow;
use url::Url;
use yew::prelude::*;
use yew_router::{route::Route, service::RouteService, Switch};

pub struct AppBoot {
    locale: Option<Locale>,
}

pub enum AppBootMsg {
    LocaleLoaded(Result<Locale, FetchFluentError>),
}

impl Component for AppBoot {
    type Message = AppBootMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        wasm_bindgen_futures::spawn_local(async move {
            let locale = Locale::load_for_user().await;
            link.send_message(AppBootMsg::LocaleLoaded(locale));
        });

        Self { locale: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppBootMsg::LocaleLoaded(res) => {
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
                <App locale=locale.clone()/>
            }
        } else {
            html! {}
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Switch)]
pub enum AppRoute {
    #[to = "/games"]
    Games,
    #[to = "/"]
    Home,
}

pub type NavigateCallback = Callback<AppRoute>;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AppProps {
    pub locale: Locale,
}

pub enum AppMsg {
    RouteChanged(Route),
    ChangeRoute(AppRoute),
}

pub struct App {
    props: AppProps,
    link: ComponentLink<Self>,
    route_service: RouteService,
    route: Route,
    api: SharedAPI,
}

impl App {
    pub fn create_navigate_callback(&self) -> NavigateCallback {
        let link = self.link.clone();
        Callback::from(move |route: AppRoute| link.send_message(AppMsg::ChangeRoute(route)))
    }
}

impl Component for App {
    type Message = AppMsg;
    type Properties = AppProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        route_service.register_callback(link.callback(AppMsg::RouteChanged));
        let route = route_service.get_route();

        // TODO make this configurable
        let mut api_url = crate::js::get_url().unwrap();
        api_url.set_port(Some(8000)).unwrap();
        api_url.set_path("/");

        Self {
            props,
            link,
            route_service,
            route,
            api: SharedAPI::from(API::new(api_url)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use AppMsg::*;
        match msg {
            RouteChanged(route) => {
                self.route = route;
                true
            }
            ChangeRoute(route) => {
                let route: Route = route.into();
                self.route_service.set_route(&route, ());
                self.route = route;
                true
            }
        }
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
        let nav_cb = self.create_navigate_callback();

        use AppRoute::*;
        match AppRoute::switch(self.route.clone()) {
            Some(Home) => html! {<StartView locale=props.locale.clone() navigate=nav_cb/>},
            Some(Games) => html! {<GamesView locale=props.locale.clone() api=self.api.clone()/>},
            None => html! {404},
        }
    }
}
