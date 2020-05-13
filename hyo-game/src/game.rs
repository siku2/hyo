use crate::{
    locale::{self, LocaleMap},
    Assets,
    Manifest,
};
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum GameLoadError {
    #[error("invalid path")]
    InvalidPath,
    #[error("invalid manifest file: {0}")]
    ManifestError(anyhow::Error),
    #[error("assets error: {0}")]
    AssetsError(anyhow::Error),
    #[error("locales error: {0}")]
    LocalesError(anyhow::Error),
}

#[derive(Debug)]
pub struct Game {
    pub base_dir: PathBuf,
    pub manifest: Manifest,
    pub assets: Assets,
    pub locales: LocaleMap,
}

impl Game {
    pub fn load(dir: &Path) -> Result<Self, GameLoadError> {
        let dir = dir.canonicalize().map_err(|_| GameLoadError::InvalidPath)?;
        let hyo_file = dir.join("hyo.toml");
        let manifest = Manifest::load(&hyo_file).map_err(GameLoadError::ManifestError)?;
        let assets = Assets::load(&dir.join("assets")).map_err(GameLoadError::AssetsError)?;
        let locales = locale::load_locale_map(
            &dir.join("locale"),
            manifest.metadata.fallback_language.clone(),
        )
        .map_err(GameLoadError::LocalesError)?;

        Ok(Self {
            base_dir: dir,
            manifest,
            assets,
            locales,
        })
    }
}
