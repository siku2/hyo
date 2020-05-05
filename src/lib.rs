mod app;
mod components;
mod game;
mod locale;

use wasm_bindgen::prelude::*;
use wasm_logger;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();

    Ok(())
}
