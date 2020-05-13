use hyo_bridge::rest::{GameInfo, GameInfoList};
use reqwest::Client;
use url::{ParseError, Url};

#[derive(Debug, thiserror::Error)]
pub enum APIError {
    #[error("invalid url: {0}")]
    InvalidURL(#[from] ParseError),

    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

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
