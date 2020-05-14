use hyo_bridge::rest::GameInfoList;
use reqwest::Client;
use std::{ops::Deref, rc::Rc};
use url::{ParseError, Url};

#[derive(Debug, thiserror::Error)]
pub enum APIError {
    #[error("invalid url: {0}")]
    InvalidURL(#[from] ParseError),

    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

#[derive(Debug)]
pub struct API {
    api_url: Url,
    client: Client,
}

impl API {
    pub fn new(api_url: Url) -> Self {
        Self {
            api_url,
            client: Client::new(),
        }
    }

    fn url(&self, path: &str) -> Result<Url, ParseError> {
        self.api_url.join(path)
    }

    fn game_url(&self, game_id: &str) -> Result<Url, ParseError> {
        self.url(&format!("games/{}", game_id))
    }

    fn game_asset_url(&self, game_id: &str, asset_name: &str) -> Result<Url, ParseError> {
        self.url(&format!("games/{}/assets/{}/", game_id, asset_name))
    }

    pub fn game_asset_url_str(&self, game_id: &str, asset_name: &str) -> Option<String> {
        self.game_asset_url(game_id, asset_name)
            .map(|u| u.to_string())
            .ok()
    }

    pub async fn get_games(&self) -> Result<GameInfoList, APIError> {
        self.client
            .get(self.url("games")?)
            .send()
            .await?
            .json()
            .await
            .map_err(APIError::from)
    }
}

#[derive(Clone, Debug)]
pub struct SharedAPI(Rc<API>);

impl Deref for SharedAPI {
    type Target = API;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for SharedAPI {
    fn eq(&self, other: &SharedAPI) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl From<API> for SharedAPI {
    fn from(api: API) -> Self {
        Self(Rc::new(api))
    }
}
