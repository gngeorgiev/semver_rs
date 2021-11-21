use crate::error::Error;

use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Allows building an [Options](crate::Options) instance.
/// ## Example
/// ```
/// # use semver_rs::Options;
/// let opts = Options::builder().loose(true).include_prerelease(true).build();
/// ```
#[derive(Debug)]
pub struct OptionsBuilder {
    opts: Options,
}

impl OptionsBuilder {
    /// Sets the `loose` option. Refer to [Options.loose](crate::Options::loose).
    pub fn loose(mut self, loose: bool) -> Self {
        self.opts.loose = loose;
        self
    }

    /// Sets the `include_prerelease` option. Refer to [Options.include_prerelease](crate::Options::include_prerelease).
    pub fn include_prerelease(mut self, include: bool) -> Self {
        self.opts.include_prerelease = include;
        self
    }

    pub fn build(self) -> Options {
        self.opts
    }
}

/// Allows to configure the parsing of semver strings, same as the [node-semver](https://github.com/npm/node-semver#functions) package.
/// All options are false by default.
/// ## Example
/// ```
/// # use semver_rs::{Options, Version, Error};
/// let opts = Options::builder().loose(true).include_prerelease(true).build();
/// //or
/// let opts = Options { loose: true, include_prerelease: true };
///
/// Version::new("1.2.3").with_options(opts).parse()?;
/// # Ok::<(), Error>(())
/// ```
#[derive(Default, Clone, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Options {
    /// Be more forgiving about not-quite-valid semver strings.
    /// Any resulting output will always be 100% strict compliant.
    pub loose: bool,

    /// Set to suppress the [default behavior](https://github.com/npm/node-semver#prerelease-tags) of excluding prerelease tagged
    /// versions from ranges unless they are explicitly opted into.
    pub include_prerelease: bool,
}

impl Options {
    /// Returns a builder that allows building a [Options](crate::Options) instance.    
    pub fn builder() -> OptionsBuilder {
        OptionsBuilder {
            opts: Options {
                include_prerelease: false,
                loose: false,
            },
        }
    }
}

/// A Builder that helps create instances of [Version](crate::Version) and [Range](crate::Range)
/// by also optionally supplying [Options](crate::Options).
#[derive(Debug)]
pub struct Builder<'a, T>
where
    T: Parseable<'a>,
{
    _phantom: std::marker::PhantomData<T>,
    opts: Option<Options>,
    input: &'a str,
}

impl<'a, T> Builder<'a, T>
where
    T: Parseable<'a>,
{
    pub fn new(input: &'a str) -> Self {
        Builder {
            _phantom: PhantomData::default(),
            opts: None,
            input,
        }
    }

    pub fn with_options_maybe(mut self, opts: Option<Options>) -> Self {
        self.opts = opts;
        self
    }

    pub fn with_options(mut self, opts: Options) -> Self {
        self.opts = Some(opts);
        self
    }

    pub fn parse(self) -> Result<T, Error> {
        T::parse(self.input, self.opts)
    }
}

pub trait Parseable<'p>: Sized {
    fn parse(input: &'p str, opts: Option<Options>) -> Result<Self, Error>;
}
