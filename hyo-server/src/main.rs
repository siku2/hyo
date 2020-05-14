#![feature(proc_macro_hygiene, decl_macro)]

use crate::{
    library::GameLibrary,
    session::{Session, SessionServer},
};
use hyo_bridge::rest::{GameInfo, GameInfoList};
use hyo_fluent::LanguageIdentifier;
use hyo_game::Game;
use rocket::{
    http::{Method, RawStr, Status},
    request::{self, FromRequest},
    response::NamedFile,
    Outcome,
    Request,
    State,
};
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use serde::Serialize;
use std::path::PathBuf;
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

fn game_info_from_game(game: &Game, accept_language: &AcceptLanguage) -> GameInfo {
    let bundles = game.locales.negotiate(&accept_language.0);
    GameInfo {
        id: game.manifest.id.clone(),
        name: bundles.localize("hyo-game-name", None).into_owned(),
        description: bundles.localize("hyo-game-description", None).into_owned(),
    }
}

#[rocket::get("/games")]
fn get_games<'a>(
    accept_language: AcceptLanguage,
    game_library: State<'a, GameLibrary>,
) -> Json<GameInfoList> {
    let games = game_library
        .inner()
        .values()
        .map(|game| game_info_from_game(game, &accept_language))
        .collect();
    Json(games)
}

#[rocket::get("/games/<id>")]
fn get_game<'a>(
    id: &RawStr,
    accept_language: AcceptLanguage,
    game_library: State<'a, GameLibrary>,
) -> Option<Json<GameInfo>> {
    game_library
        .inner()
        .get(id.as_str())
        .map(|game| game_info_from_game(game, &accept_language))
        .map(Json)
}

#[rocket::get("/games/<game_id>/assets/<name..>")]
fn get_game_asset(
    game_id: &RawStr,
    name: PathBuf,
    game_library: State<GameLibrary>,
) -> Option<NamedFile> {
    game_library
        .inner()
        .get(game_id.as_str())?
        .assets
        .get(name.to_str()?)
        .and_then(|asset| NamedFile::open(&asset.path).ok())
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
fn get_sessions<'a>(session_manager: State<'a, SessionServer>) -> Json<Vec<SessionModel<'a>>> {
    let sessions = session_manager
        .inner()
        .iter_public_sessions()
        .map(SessionModel::from)
        .collect();
    Json(sessions)
}

#[rocket::post("/sessions")]
fn create_session<'a>(_session_manager: State<'a, SessionServer>) -> Json<SessionModel<'a>> {
    unimplemented!()
}

#[rocket::post("/sessions/join")]
fn join_session<'a>(_session_manager: State<'a, SessionServer>) -> Json<SessionModel<'a>> {
    unimplemented!()
}

fn build_cors() -> Result<Cors, rocket_cors::Error> {
    let allowed_methods = vec![Method::Get]
        .drain(..)
        .map(rocket_cors::Method::from)
        .collect();

    // TODO restrict access at least a bit
    CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        send_wildcard: true,
        ..Default::default()
    }
    .to_cors()
}

fn main() -> Result<(), anyhow::Error> {
    // init logging and config
    let rocket = rocket::ignite();

    let game_library = library::load("games")?;
    let session_manager = SessionServer::default();

    rocket
        .manage(game_library)
        .manage(session_manager)
        .mount(
            "/",
            rocket::routes![
                get_games,
                get_game,
                get_game_asset,
                get_sessions,
                create_session,
                join_session
            ],
        )
        .attach(build_cors()?)
        .launch();

    Ok(())
}
