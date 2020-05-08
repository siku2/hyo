use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
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
