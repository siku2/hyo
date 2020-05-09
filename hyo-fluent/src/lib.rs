use fluent::{FluentArgs, FluentBundle, FluentError, FluentMessage, FluentResource};
use fluent_langneg::NegotiationStrategy;
use std::borrow::{Borrow, Cow};
use thiserror::Error;
pub use unic_langid::LanguageIdentifier;

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

pub struct FluentBundles<R>(pub Vec<FluentBundle<R>>);

impl<R: Borrow<FluentResource>> FluentBundles<R> {
    pub fn new(bundles: Vec<FluentBundle<R>>) -> Self {
        Self(bundles)
    }

    fn iter_bundles(&self) -> IterFluentBundles<impl Iterator<Item = &FluentBundle<R>>> {
        IterFluentBundles(self.0.iter())
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

pub fn negotiate_languages<'s>(
    available: &'s [LanguageIdentifier],
    requested: &[LanguageIdentifier],
    fallback: &'s LanguageIdentifier,
) -> Vec<&'s LanguageIdentifier> {
    fluent_langneg::negotiate_languages(
        requested,
        &available,
        Some(fallback),
        NegotiationStrategy::Filtering,
    )
}
