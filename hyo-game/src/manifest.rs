use hyo_fluent::LanguageIdentifier;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
    pub id: String,
    pub version: Version,
    pub authors: Vec<String>,

    pub dependencies: Dependencies,
    pub metadata: Metadata,
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self, anyhow::Error> {
        let raw = fs::read(path)?;
        toml::from_slice(&raw).map_err(|e| e.into())
    }
}
