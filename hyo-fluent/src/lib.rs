pub use fluent::{
    concurrent::FluentBundle,
    FluentArgs,
    FluentError,
    FluentMessage,
    FluentResource,
};
use fluent_langneg::NegotiationStrategy;
use std::{
    borrow::{Borrow, Cow},
    fmt,
    marker::PhantomData,
};
use thiserror::Error;
pub use unic_langid::LanguageIdentifier;

pub fn negotiate_languages<
    's,
    L: AsRef<LanguageIdentifier> + 's,
    LP: AsRef<LanguageIdentifier> + PartialEq + 's,
>(
    requested: &[L],
    available: &'s [LP],
    fallback: &'s LP,
) -> Vec<&'s LP> {
    fluent_langneg::negotiate_languages(
        requested,
        available,
        Some(fallback),
        NegotiationStrategy::Filtering,
    )
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

pub struct IterFluentBundles<I>(I);

impl<'s, R: Borrow<FluentResource> + 's, I: Iterator<Item = &'s FluentBundle<R>> + 's>
    IterFluentBundles<I>
{
    pub fn get_message(self, id: &str) -> Option<FluentMessage<'s>> {
        self.0.filter_map(|bundle| bundle.get_message(id)).next()
    }

    pub fn format(
        self,
        id: &str,
        args: Option<&'s FluentArgs>,
    ) -> Result<Cow<'s, str>, FluentFormatError> {
        let (bundle, msg) = self
            .0
            .filter_map(|bundle| bundle.get_message(id).map(|msg| (bundle, msg)))
            .next()
            .ok_or(FluentFormatError::NotFound)?;
        format_message(bundle, &msg, args)
    }

    pub fn localize(self, id: &'s str, args: Option<&'s FluentArgs>) -> Cow<'s, str> {
        self.format(id, args).unwrap_or_else(|_| id.into())
    }
}

pub struct GenericFluentBundles<B, R>(pub Vec<B>, PhantomData<R>);

impl<B: Borrow<FluentBundle<R>>, R: Borrow<FluentResource>> GenericFluentBundles<B, R> {
    pub fn new(bundles: Vec<B>) -> Self {
        Self(bundles, PhantomData)
    }

    fn iter_bundles(&self) -> IterFluentBundles<impl Iterator<Item = &FluentBundle<R>>> {
        IterFluentBundles(self.0.iter().map(Borrow::borrow))
    }

    pub fn get_message<'a>(&'a self, id: &'a str) -> Option<FluentMessage<'a>> {
        self.iter_bundles().get_message(id)
    }

    pub fn format<'a>(
        &'a self,
        id: &'a str,
        args: Option<&'a FluentArgs>,
    ) -> Result<Cow<'a, str>, FluentFormatError> {
        self.iter_bundles().format(id, args)
    }

    pub fn localize<'a>(&'a self, id: &'a str, args: Option<&'a FluentArgs>) -> Cow<'a, str> {
        self.iter_bundles().localize(id, args)
    }
}

pub type OwnedFluentBundles<R> = GenericFluentBundles<FluentBundle<R>, R>;
pub type BorrowedFluentBundles<'a, R> = GenericFluentBundles<&'a FluentBundle<R>, R>;

pub struct LocaleMap {
    languages: Vec<LanguageIdentifier>,
    bundles: Vec<FluentBundle<FluentResource>>,
    pub fallback: LanguageIdentifier,
}

impl LocaleMap {
    fn with_capacity(fallback: LanguageIdentifier, capacity: usize) -> Self {
        Self {
            languages: Vec::with_capacity(capacity),
            bundles: Vec::with_capacity(capacity),
            fallback,
        }
    }

    pub fn new_with_fallback(
        bundles: Vec<FluentBundle<FluentResource>>,
        fallback: LanguageIdentifier,
    ) -> Self {
        let mut inst = Self::with_capacity(fallback, bundles.len());
        for bundle in bundles {
            inst.add_bundle(bundle);
        }
        inst
    }

    pub fn has_fallback(&self) -> bool {
        self.get_bundle(&self.fallback).is_some()
    }

    fn add_bundle(&mut self, bundle: FluentBundle<FluentResource>) -> bool {
        let langid = match bundle.locales.first() {
            Some(v) => v,
            None => return false,
        };

        let pos = self.languages.binary_search(langid).unwrap_or_else(|i| i);
        self.languages.insert(pos, langid.clone());
        self.bundles.insert(pos, bundle);
        true
    }

    fn get_bundle(&self, langid: &LanguageIdentifier) -> Option<&FluentBundle<FluentResource>> {
        self.languages
            .binary_search(langid)
            .ok()
            .and_then(|i| self.bundles.get(i))
    }

    fn iter_bundles<L: AsRef<LanguageIdentifier>>(
        &self,
        langids: impl IntoIterator<Item = L>,
    ) -> impl Iterator<Item = &FluentBundle<FluentResource>> {
        langids
            .into_iter()
            .flat_map(move |langid| self.get_bundle(langid.as_ref()))
    }

    fn get_bundles<L: AsRef<LanguageIdentifier>>(
        &self,
        langids: impl IntoIterator<Item = L>,
    ) -> Vec<&FluentBundle<FluentResource>> {
        self.iter_bundles(langids).collect()
    }

    pub fn negotiate<L: AsRef<LanguageIdentifier>>(
        &self,
        requested: &[L],
    ) -> BorrowedFluentBundles<FluentResource> {
        let langids = negotiate_languages(requested, &self.languages, &self.fallback);
        GenericFluentBundles::new(self.get_bundles(langids))
    }
}

impl fmt::Debug for LocaleMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocaleMap")
            .field("languages", &self.languages)
            .field("fallback", &self.fallback)
            .finish()
    }
}
