use crate::manifest::Manifest;
use fluent::{FluentBundle, FluentResource};
use hyo_fluent::{FluentBundles, LanguageIdentifier};
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Clone, Debug)]
pub struct Locales {
    locale_dir: PathBuf,
    available: Vec<LanguageIdentifier>,
    fallback: LanguageIdentifier,
}

impl Locales {
    pub fn load(locale_dir: &Path, fallback: LanguageIdentifier) -> Result<Self, anyhow::Error> {
        let mut available = Vec::new();
        for entry in locale_dir.read_dir()? {
            let dir_path = entry?.path();
            let stem = dir_path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("invalid file name: {:?}", dir_path))?;
            available.push(LanguageIdentifier::from_str(stem)?);
        }

        Ok(Self {
            locale_dir: locale_dir.to_path_buf(),
            available,
            fallback,
        })
    }

    fn load_resource(&self, langid: &LanguageIdentifier) -> Result<FluentResource, anyhow::Error> {
        let s = fs::read_to_string(
            self.locale_dir
                .join(langid.to_string())
                .with_extension("ftl"),
        )?;
        FluentResource::try_new(s)
            .map_err(|(_, errs)| anyhow::anyhow!("failed to load fluent resource: {:?}", errs))
    }

    fn load_bundle(
        &self,
        langid: LanguageIdentifier,
    ) -> Result<FluentBundle<FluentResource>, anyhow::Error> {
        let resource = self.load_resource(&langid)?;
        let mut bundle = FluentBundle::new(&[langid]);
        bundle.add_resource(resource).unwrap();
        Ok(bundle)
    }

    fn load_bundles(
        &self,
        langids: impl Iterator<Item = LanguageIdentifier>,
    ) -> Result<FluentBundles<FluentResource>, anyhow::Error> {
        let bundles: Vec<_> = langids
            .map(|langid| self.load_bundle(langid))
            .collect::<Result<_, _>>()?;
        Ok(FluentBundles::new(bundles))
    }

    pub fn get_bundles(
        &self,
        langids: &[LanguageIdentifier],
    ) -> Result<FluentBundles<FluentResource>, anyhow::Error> {
        // TODO store this
        let langids = hyo_fluent::negotiate_languages(&self.available, langids, &self.fallback);
        self.load_bundles(langids.into_iter().cloned())
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub base_dir: PathBuf,
    pub manifest: Manifest,
    pub locales: Locales,
}

impl Game {
    // TODO move away from anyhow error
    pub fn load(dir: &Path) -> Result<Self, anyhow::Error> {
        let dir = dir.canonicalize()?;
        let hyo_file = dir.join("hyo.toml");
        // TODO check if file and other things
        let manifest = Manifest::load(&hyo_file)?;
        let locales = Locales::load(
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
