#![feature(proc_macro_hygiene, decl_macro)]

use hyo_game::Game;
use rocket::{
    http::{RawStr, Status},
    request::{self, FromRequest},
    Outcome,
    Request,
    State,
};
use rocket_contrib::json::Json;
use serde::Serialize;

use unic_langid::LanguageIdentifier;
use uuid::Uuid;

mod library;

struct AcceptLanguage(pub Vec<LanguageIdentifier>);

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = &'static str;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let languages = match request.headers().get_one("accept-language") {
            Some(v) => v,
            None => {
                return Outcome::Failure((Status::BadRequest, "missing Accept-Language header"))
            }
        };

        Outcome::Success(Self(fluent_langneg::parse_accepted_languages(languages)))
    }
}

#[derive(Serialize)]
struct GameModel<'s> {
    id: &'s str,
    name: String,
    description: String,
}

impl<'s> GameModel<'s> {
    fn from_game(game: &'s Game, requested_languages: &[LanguageIdentifier]) -> Self {
        let bundles = game.locales.get_bundles(requested_languages).unwrap();
        let name = bundles.localize("hyo-game-name", None).into_owned();
        let description = bundles.localize("hyo-game-description", None).into_owned();
        Self {
            id: &game.manifest.id,
            name,
            description,
        }
    }
    fn from_game_old(game: &'s Game) -> Self {
        Self {
            id: &game.manifest.id,
            name: "".into(),
            description: "".into(),
        }
    }
}

#[rocket::get("/games")]
fn get_games<'a>(
    accept_language: AcceptLanguage,
    game_library: State<'a, library::GameLibrary>,
) -> Json<Vec<GameModel<'a>>> {
    let games = game_library
        .inner()
        .values()
        .map(|game| GameModel::from_game(game, &accept_language.0))
        .collect();
    Json(games)
}

#[rocket::get("/games/<id>")]
fn get_game<'a>(
    id: &RawStr,
    game_library: State<'a, library::GameLibrary>,
) -> Option<Json<GameModel<'a>>> {
    game_library
        .inner()
        .get(id.as_str())
        .map(GameModel::from_game_old)
        .map(Json)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionSettings {
    pub public: bool,
    pub max_players: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
    pub id: Uuid,
    pub game_id: String,
}

fn main() -> Result<(), anyhow::Error> {
    // init logging and config
    let rocket = rocket::ignite();

    let game_library = library::load("games")?;

    rocket
        .manage(game_library)
        .mount("/", rocket::routes![get_games, get_game])
        .launch();

    Ok(())
}
