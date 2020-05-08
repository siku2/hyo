#![feature(proc_macro_hygiene, decl_macro)]

use hyo_game::Manifest;
use rocket::{http::RawStr, State};
use rocket_contrib::json::Json;
use serde::Serialize;

mod library;

#[derive(Serialize)]
struct GameModel<'m> {
    id: &'m str,
}

impl<'m> From<&'m Manifest> for GameModel<'m> {
    fn from(manifest: &'m Manifest) -> Self {
        Self {
            id: &manifest.game.id,
        }
    }
}

#[rocket::get("/games")]
fn get_games<'a>(game_library: State<'a, library::GameLibrary>) -> Json<Vec<GameModel<'a>>> {
    let games = game_library.inner().iter().map(GameModel::from).collect();
    Json(games)
}

#[rocket::get("/games/<id>")]
fn get_game<'a>(
    id: &RawStr,
    game_library: State<'a, library::GameLibrary>,
) -> Option<Json<GameModel<'a>>> {
    game_library
        .inner()
        .iter()
        .find(|m| id == &m.game.id)
        .map(GameModel::from)
        .map(Json)
}

fn main() -> Result<(), anyhow::Error> {
    // init logging and config
    let rocket = rocket::ignite();

    let game_library = library::load("./games")?;

    rocket
        .manage(game_library)
        .mount("/", rocket::routes![get_games, get_game])
        .launch();

    Ok(())
}
