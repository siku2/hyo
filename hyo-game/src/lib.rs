use std::{fs, path::Path};

mod manifest;

pub use manifest::*;

// TODO move away from anyhow error
pub fn load_game(game_dir: &Path) -> Result<Manifest, anyhow::Error> {
    let hyo_file = game_dir.join("hyo.toml");
    // TODO check if file and other things
    let raw_manifest = fs::read(&hyo_file)?;
    toml::from_slice(&raw_manifest).map_err(|e| e.into())
}
