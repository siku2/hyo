#![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::RawStr;
#[macro_use]
extern crate rocket;

mod game;

#[get("/games")]
fn get_games() {}
#[get("/games/<id>")]
fn get_game(id: &RawStr) {}

fn main() {
    pretty_env_logger::init();

    let games = game::load_games("./games").unwrap();
    println!("{:?}", games);

    rocket::ignite()
        .mount("/", routes![get_games, get_game])
        .launch();
}
