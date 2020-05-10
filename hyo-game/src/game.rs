use crate::manifest::Manifest;
use hyo_fluent::{FluentBundle, FluentResource, LanguageIdentifier, LocaleMap};
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

fn load_resource_file(path: &Path) -> Result<FluentResource, anyhow::Error> {
    let s = fs::read_to_string(path)?;
    FluentResource::try_new(s)
        .map_err(|(_, errs)| anyhow::anyhow!("failed to load fluent resource: {:?}", errs))
}

fn load_bundle_file(
    path: &Path,
    langid: LanguageIdentifier,
) -> Result<FluentBundle<FluentResource>, anyhow::Error> {
    let resource = load_resource_file(path)?;
    let mut bundle = FluentBundle::new(&[langid]);
    bundle.add_resource(resource).unwrap();
    Ok(bundle)
}

fn load_locale_map(
    locale_dir: &Path,
    fallback: LanguageIdentifier,
) -> Result<LocaleMap, anyhow::Error> {
    let mut bundles = Vec::new();
    for entry in locale_dir.read_dir()? {
        let file_path = entry?.path();
        if !file_path.is_file() {
            return Err(anyhow::anyhow!("locale directory contains non-file"));
        }

        let stem = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("invalid file name: {:?}", file_path))?;
        let langid = LanguageIdentifier::from_str(stem)?;
        bundles.push(load_bundle_file(&file_path, langid)?);
    }

    let inst = LocaleMap::new_with_fallback(bundles, fallback);
    if !inst.has_fallback() {
        return Err(anyhow::anyhow!(
            "fallback language ({}) doesn't exist",
            inst.fallback
        ));
    }

    Ok(inst)
}

#[derive(Debug)]
pub struct Game {
    pub base_dir: PathBuf,
    pub manifest: Manifest,
    pub locales: LocaleMap,
}

impl Game {
    // TODO move away from anyhow error
    pub fn load(dir: &Path) -> Result<Self, anyhow::Error> {
        let dir = dir.canonicalize()?;
        let hyo_file = dir.join("hyo.toml");
        let manifest = Manifest::load(&hyo_file)?;
        let locales = load_locale_map(
            &dir.join("locale"),
            manifest.metadata.fallback_language.clone(),
        )?;

        Ok(Self {
            base_dir: dir,
            manifest,
            locales,
        })
    }
}
