#![feature(proc_macro_hygiene, decl_macro)]

use crate::{
    library::GameLibrary,
    session::{Session, SessionManager},
};
use hyo_fluent::LanguageIdentifier;
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
use uuid::Uuid;

mod library;
mod session;
mod uno;

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

        Outcome::Success(Self(hyo_fluent::parse_accepted_languages(languages)))
    }
}

#[derive(Serialize)]
struct GameModel<'s> {
    id: &'s str,
    name: String,
    description: String,
}

impl<'s> GameModel<'s> {
    fn from_game(game: &'s Game, accept_language: &AcceptLanguage) -> Self {
        let bundles = game.locales.negotiate(&accept_language.0);
        let name = bundles.localize("hyo-game-name", None).into_owned();
        let description = bundles.localize("hyo-game-description", None).into_owned();
        Self {
            id: &game.manifest.id,
            name,
            description,
        }
    }
}

#[rocket::get("/games")]
fn get_games<'a>(
    accept_language: AcceptLanguage,
    game_library: State<'a, GameLibrary>,
) -> Json<Vec<GameModel<'a>>> {
    let games = game_library
        .inner()
        .values()
        .map(|game| GameModel::from_game(game, &accept_language))
        .collect();
    Json(games)
}

#[rocket::get("/games/<id>")]
fn get_game<'a>(
    id: &RawStr,
    accept_language: AcceptLanguage,
    game_library: State<'a, GameLibrary>,
) -> Option<Json<GameModel<'a>>> {
    game_library
        .inner()
        .get(id.as_str())
        .map(|game| GameModel::from_game(game, &accept_language))
        .map(Json)
}

#[derive(Serialize)]
struct SessionModel<'s> {
    id: &'s Uuid,
    game_id: &'s str,
}

impl<'s> From<&'s Session> for SessionModel<'s> {
    fn from(s: &'s Session) -> Self {
        Self {
            id: &s.id,
            game_id: &s.game_id,
        }
    }
}

#[rocket::get("/sessions")]
fn get_sessions<'a>(session_manager: State<'a, SessionManager>) -> Json<Vec<SessionModel<'a>>> {
    let sessions = session_manager
        .inner()
        .iter_public_sessions()
        .map(SessionModel::from)
        .collect();
    Json(sessions)
}

#[rocket::post("/sessions")]
fn create_session<'a>(_session_manager: State<'a, SessionManager>) -> Json<SessionModel<'a>> {
    unimplemented!()
}

fn main() -> Result<(), anyhow::Error> {
    // init logging and config
    let rocket = rocket::ignite();

    let game_library = library::load("games")?;
    let session_manager = SessionManager::default();

    rocket
        .manage(game_library)
        .manage(session_manager)
        .mount("/", rocket::routes![get_games, get_game, get_sessions])
        .launch();

    Ok(())
}
