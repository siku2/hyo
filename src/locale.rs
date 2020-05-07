use crate::fetch;
use fluent::{FluentArgs, FluentBundle, FluentError, FluentMessage, FluentResource};
use futures::future;
use std::{
    borrow::{Borrow, Cow},
    rc::Rc,
    str::FromStr,
};
use thiserror::Error;
use unic_langid::{langid, LanguageIdentifier};
use web_sys::{Request, RequestInit, RequestMode};

static FALLBACK_LANGUAGE: LanguageIdentifier = langid!("en-GB");

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

fn get_user_languages() -> Vec<LanguageIdentifier> {
    if let Some(lang) = get_stored_language() {
        vec![lang]
    } else {
        get_browser_languages()
    }
}

#[derive(Debug, Error)]
pub enum FetchFluentError {
    #[error(transparent)]
    Fetch(#[from] fetch::FetchError),
    #[error("parse error")]
    Parse,
}

// TODO support for "natural" fallback chain like: fr-CA > fr-FR > fr > en-GB

async fn fetch_fluent_resource(
    langid: &LanguageIdentifier,
) -> Result<FluentResource, FetchFluentError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&format!("locale/{}.ftl", langid), &opts).unwrap();

    let raw = fetch::perform_text_request(request).await?;
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

async fn fetch_first_fluent_bundle(
    langids: impl IntoIterator<Item = LanguageIdentifier>,
) -> Option<FluentBundle<FluentResource>> {
    for langid in langids {
        if let Ok(bundle) = fetch_fluent_bundle(langid).await {
            return Some(bundle);
        }
    }

    None
}

#[derive(Debug, Error)]
pub enum FluentFormatError {
    #[error("message not found")]
    NotFound,
    #[error("message has no value")]
    NoValue,

    #[error("formatting error: {0:?}")]
    Format(Vec<FluentError>),
}

pub fn format_message<'a>(
    bundle: &'a FluentBundle<impl Borrow<FluentResource>>,
    msg: &FluentMessage<'a>,
    args: Option<&'a FluentArgs>,
) -> Result<Cow<'a, str>, FluentFormatError> {
    let pattern = msg.value.ok_or(FluentFormatError::NoValue)?;
    let mut errors = Vec::new();
    let s = bundle.format_pattern(pattern, args, &mut errors);
    if errors.is_empty() {
        Ok(s)
    } else {
        Err(FluentFormatError::Format(errors))
    }
}

pub struct LocaleBundles<R> {
    bundles: Vec<FluentBundle<R>>,
}

impl LocaleBundles<FluentResource> {
    pub async fn load(
        langids: impl IntoIterator<Item = LanguageIdentifier>,
    ) -> Result<Self, FetchFluentError> {
        let bundles = future::try_join_all(langids.into_iter().map(fetch_fluent_bundle)).await?;
        Ok(Self::new(bundles))
    }

    pub async fn load_first_with_fallback(
        langids: impl IntoIterator<Item = LanguageIdentifier>,
        fallback: LanguageIdentifier,
    ) -> Result<Self, FetchFluentError> {
        let mut bundles = vec![];

        if let Some(bundle) = fetch_first_fluent_bundle(langids).await {
            bundles.push(bundle);
        }

        bundles.push(fetch_fluent_bundle(fallback).await?);

        Ok(Self::new(bundles))
    }
}

impl<R: Borrow<FluentResource>> LocaleBundles<R> {
    fn new(bundles: Vec<FluentBundle<R>>) -> Self {
        Self { bundles }
    }

    fn iter_bundles(&self) -> impl Iterator<Item = &FluentBundle<R>> {
        self.bundles.iter()
    }

    fn iter_locales(&self) -> impl Iterator<Item = &LanguageIdentifier> {
        self.iter_bundles()
            .flat_map(move |bundle| bundle.locales.iter())
    }

    fn iter_bundles_message<'a>(
        &'a self,
        id: &'a str,
    ) -> impl Iterator<Item = (&FluentBundle<R>, FluentMessage<'a>)> {
        self.iter_bundles()
            .filter_map(move |bundle| bundle.get_message(id).map(|msg| (bundle, msg)))
    }

    pub fn get_message<'a>(&'a self, id: &'a str) -> Option<FluentMessage<'a>> {
        self.iter_bundles_message(id).map(|(_, msg)| msg).next()
    }

    pub fn format<'a>(
        &'a self,
        id: &'a str,
        args: Option<&'a FluentArgs>,
    ) -> Result<Cow<'a, str>, FluentFormatError> {
        let (bundle, msg) = self
            .iter_bundles_message(id)
            .next()
            .ok_or(FluentFormatError::NotFound)?;
        format_message(bundle, &msg, args)
    }

    pub fn format_or_id<'a>(&'a self, id: &'a str, args: Option<&'a FluentArgs>) -> Cow<'a, str> {
        self.format(id, args).unwrap_or_else(|err| {
            log::warn!("failed to format message {}: {}", id, err);
            id.into()
        })
    }
}

#[derive(Clone)]
pub struct Locale {
    builtin: Rc<LocaleBundles<FluentResource>>,
}

impl Locale {
    fn new(builtin: Rc<LocaleBundles<FluentResource>>) -> Self {
        Self { builtin }
    }

    pub async fn load_for_user() -> Result<Self, FetchFluentError> {
        let bundles = LocaleBundles::load_first_with_fallback(
            get_user_languages(),
            FALLBACK_LANGUAGE.clone(),
        )
        .await?;
        Ok(Self::new(Rc::new(bundles)))
    }

    pub fn localize<'a>(&'a self, id: &'a str, args: Option<&'a FluentArgs<'a>>) -> Cow<'a, str> {
        self.builtin.format_or_id(id, args)
    }
}

impl PartialEq for Locale {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.builtin, &other.builtin)
    }
}
