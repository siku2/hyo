use futures::future;
use hyo_fluent::{
    FluentArgs,
    FluentBundle,
    FluentResource,
    LanguageIdentifier,
    OwnedFluentBundles,
};
use std::{borrow::Cow, rc::Rc, str::FromStr};
use thiserror::Error;

static FALLBACK_LANGUAGE: LanguageIdentifier = hyo_fluent::langid!("en-GB");
const SUPPORTED_LANGUAGES: &[LanguageIdentifier] = hyo_fluent::langids!["de-DE", "en-GB"];

fn get_browser_languages() -> Vec<LanguageIdentifier> {
    let navigator = web_sys::window().unwrap().navigator();
    navigator
        .languages()
        .iter()
        .filter_map(|lang_val| {
            let lang = lang_val.as_string()?;
            match LanguageIdentifier::from_str(&lang) {
                Ok(id) => Some(id),
                Err(e) => {
                    log::error!("failed to parse language `{}`: {}", lang, e);
                    None
                }
            }
        })
        .collect()
}

fn get_stored_language() -> Option<LanguageIdentifier> {
    // TODO delegate to storage module
    let local_storage = web_sys::window()?.local_storage().ok()??;
    let lang = local_storage.get_item("language").ok()??;
    LanguageIdentifier::from_str(&lang).ok()
}

fn get_user_languages() -> Vec<&'static LanguageIdentifier> {
    let mut langs = get_browser_languages();
    if let Some(lang) = get_stored_language() {
        langs.insert(0, lang);
    }
    hyo_fluent::negotiate_languages(&langs, &SUPPORTED_LANGUAGES, &FALLBACK_LANGUAGE)
}

#[derive(Debug, Error)]
pub enum FetchFluentError {
    #[error(transparent)]
    Fetch(#[from] reqwest::Error),
    #[error("parse error")]
    Parse,
}

async fn fetch_fluent_resource(
    langid: &LanguageIdentifier,
) -> Result<FluentResource, FetchFluentError> {
    let raw = reqwest::get(&format!("locale/{}.ftl", langid)).await?.text().await?;
    FluentResource::try_new(raw).map_err(|_| FetchFluentError::Parse)
}

async fn fetch_fluent_bundle(
    langid: LanguageIdentifier,
) -> Result<FluentBundle<FluentResource>, FetchFluentError> {
    let resource = fetch_fluent_resource(&langid).await?;
    let mut bundle = FluentBundle::new(&[langid]);
    bundle.add_resource(resource).unwrap();
    Ok(bundle)
}

#[derive(Clone)]
pub struct Locale {
    pub builtin: Rc<OwnedFluentBundles<FluentResource>>,
}

impl Locale {
    fn new(builtin: Rc<OwnedFluentBundles<FluentResource>>) -> Self {
        Self { builtin }
    }

    pub async fn load_for_user() -> Result<Self, FetchFluentError> {
        let bundles: Vec<_> = future::join_all(
            get_user_languages()
                .drain(..)
                .cloned()
                .map(fetch_fluent_bundle),
        )
        .await
        .drain(..)
        .filter_map(|r| match r {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("failed to load bundle: {}", e);
                None
            }
        })
        .collect();

        Ok(Self::new(Rc::new(OwnedFluentBundles::new(bundles))))
    }

    pub fn localize<'a>(&'a self, id: &'a str, args: Option<&'a FluentArgs<'a>>) -> Cow<'a, str> {
        self.builtin.localize(id, args)
    }
}

impl PartialEq for Locale {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.builtin, &other.builtin)
    }
}
