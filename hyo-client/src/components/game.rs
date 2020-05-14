use super::image::ImageFallback;
use crate::api::{APIError, SharedAPI};
use hyo_bridge::rest::{GameInfo, GameInfoList};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct GameCardProps {
    pub api: SharedAPI,
    pub info: Rc<GameInfo>,
}

pub struct GameCard {
    props: GameCardProps,
}

impl Component for GameCard {
    type Message = ();
    type Properties = GameCardProps;

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
        let info = &props.info;

        let banner_url = props
            .api
            .game_asset_url_str(&info.id, "banner")
            .unwrap_or_default();

        // TODO fallback image

        html! {
            <div class="card">
                <ImageFallback class="card__banner" src=banner_url fallback_src=""/>
                <h2 class="card__title">{ info.name.clone() }</h2>
                <p class="card__description">{ info.description.clone() }</p>
            </div>
        }
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct GamesListProps {
    pub api: SharedAPI,
}

pub enum GamesListMessage {
    GamesLoaded(Result<GameInfoList, APIError>),
}

pub struct GamesList {
    props: GamesListProps,
    link: ComponentLink<Self>,
    loading: bool,
    games: Vec<Rc<GameInfo>>,
    error: bool,
}

impl GamesList {
    fn new(props: <Self as Component>::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            loading: false,
            games: Vec::new(),
            error: false,
        }
    }

    fn start_get_games(&mut self) {
        let api = self.props.api.clone();
        let link = self.link.clone();
        self.loading = true;
        wasm_bindgen_futures::spawn_local(async move {
            let res = api.get_games().await;
            link.send_message(GamesListMessage::GamesLoaded(res));
        });
    }
}

impl Component for GamesList {
    type Message = GamesListMessage;
    type Properties = GamesListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut inst = Self::new(props, link);
        inst.start_get_games();
        inst
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use GamesListMessage::*;

        match msg {
            GamesLoaded(res) => {
                match res {
                    Ok(mut games) => {
                        self.games = games.drain(..).map(Rc::new).collect();
                        self.error = false;
                    }
                    Err(err) => {
                        log::error!("failed to load games: {}", err);
                        self.error = true;
                    }
                }

                self.loading = false;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            // TODO redo games request only if api changed
            self.start_get_games();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let card_it = self.games.iter().map(|info| {
            html! {
                <GameCard api=self.props.api.clone() info=info/>
            }
        });

        // TODO render error
        html! {
            <div>
                { for card_it }
            </div>
        }
    }
}
