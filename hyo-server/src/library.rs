use hyo_game::{self, Manifest};
use std::{fs, path::Path};

pub type GameLibrary = Vec<Manifest>;

pub fn load(path: impl AsRef<Path>) -> Result<GameLibrary, anyhow::Error> {
    let mut games = Vec::new();

    for entry in fs::read_dir(path)? {
        let dir_entry = match entry {
            Ok(v) => v,
            Err(e) => {
                log::error!("ignoring entry: {}", e);
                continue;
            }
        };

        let dir_path = dir_entry.path();

        let game = match hyo_game::load_game(&dir_path) {
            Ok(v) => v,
            Err(e) => {
                log::error!("failed to load game at {:?}: {}", dir_path, e);
                continue;
            }
        };
        games.push(game);
    }

    Ok(games)
}
