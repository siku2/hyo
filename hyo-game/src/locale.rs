pub use hyo_fluent::LocaleMap;
use hyo_fluent::{FluentBundle, FluentResource, LanguageIdentifier};
use std::{fs, path::Path, str::FromStr};

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

pub fn load_locale_map(
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
