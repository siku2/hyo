use hyo_game::Game;
use std::{collections::HashMap, fs, path::Path};

pub type GameLibrary = HashMap<String, Game>;

pub fn load(path: impl AsRef<Path>) -> Result<GameLibrary, anyhow::Error> {
    let mut games = GameLibrary::new();

    for entry in fs::read_dir(path)? {
        let dir_entry = match entry {
            Ok(v) => v,
            Err(e) => {
                log::error!("ignoring entry: {}", e);
                continue;
            }
        };

        let dir_path = dir_entry.path();

        let game = match Game::load(&dir_path) {
            Ok(v) => v,
            Err(e) => {
                log::error!("failed to load game at {:?}: {}", dir_path, e);
                continue;
            }
        };
        games.insert(game.manifest.id.clone(), game);
    }

    Ok(games)
}
