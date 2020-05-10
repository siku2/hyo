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

        let game_id = &game.manifest.id;
        if let Some(existing) = games.get(game_id) {
            log::error!(
                "id `{}` for {:?} already used by {:?}",
                game_id,
                game,
                existing
            );
            continue;
        }

        games.insert(game_id.clone(), game);
    }

    Ok(games)
}
