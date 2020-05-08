use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use unic_langid::LanguageIdentifier;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Game {
    pub id: String,
    pub version: Version,
    pub authors: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Dependencies {
    #[serde(default = "VersionReq::any")]
    pub hyo_server: VersionReq,
    #[serde(default = "VersionReq::any")]
    pub hyo_client: VersionReq,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Metadata {
    pub fallback_language: LanguageIdentifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Manifest {
    pub game: Game,
    pub dependencies: Dependencies,
    pub metadata: Metadata,
}

fn load_game(game_dir: &Path) -> Result<Manifest, anyhow::Error> {
    let hyo_file = game_dir.join("hyo.toml");
    // TODO check if file and other things
    let raw_manifest = fs::read(&hyo_file)?;
    toml::from_slice(&raw_manifest).map_err(|e| e.into())
}

pub fn load_games(path: impl AsRef<Path>) -> Result<Vec<Manifest>, anyhow::Error> {
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

        let game = match load_game(&dir_path) {
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
