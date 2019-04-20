use crate::builder::{Builder, Options, Parseable};
use crate::error::Error;
use crate::expressions::{VERSION, VERSION_LOOSE};
use crate::util::compare_identifiers;

use std::{cmp::Ordering, fmt, str};

/// A `version` is described by the `v2.0.0` specification found at [semver](https://semver.org/).
///
/// A leading `=` or `v` character is stripped off and ignored.
#[derive(Default, Clone, Debug)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub prerelease: Option<Vec<String>>,

    any: bool,
    empty: bool,
}

impl<'p> Parseable<'p> for Version {
    fn parse(comp: &'p str, opts: Option<Options>) -> Result<Self, Error> {
        let loose = opts.unwrap_or_default().loose;

        let trimmed = comp.trim();
        let cap = match loose {
            true => VERSION_LOOSE.captures(trimmed),
            false => VERSION.captures(trimmed),
        };
        let cap = match cap {
            Some(cap) => cap,
            None => return Ok(Version::empty()),
        };

        let v = if cap.get(1).map_or("", |v| v.as_str()) == "" {
            Version::any()
        } else {
            let major = cap.get(1).map_or("0", |v| v.as_str());
            let minor = cap.get(2).map_or("0", |v| v.as_str());
            let patch = cap.get(3).map_or("0", |v| v.as_str());
            let prerelease = cap.get(4).map_or(None, |v| Some(v.as_str().to_owned()));
            Version::from_parts(major.parse()?, minor.parse()?, patch.parse()?, prerelease)
        };

        Ok(v)
    }
}

impl<'p> Version {
    /// Construct a new Version e.g `1.2.4`.
    pub fn new(ver: &'p str) -> Builder<'p, Self> {
        Builder::new(ver)
    }

    /// Constructs a Version that matches any other.
    pub fn any() -> Self {
        Version {
            empty: false,
            any: true,
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: None,
        }
    }

    /// Constructs an empty Version. It's the equivalent of `Version::new("")`.
    pub fn empty() -> Self {
        Version {
            empty: true,
            any: false,
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: None,
        }
    }

    /// Constructs a version from it's already parsed parts, e.g. `Version::from_parts(1, 2, 3)`.
    pub fn from_parts(major: i32, minor: i32, patch: i32, prerelease: Option<String>) -> Self {
        let prerelease = match prerelease {
            Some(pre) => {
                let split: Vec<&str> = pre.split(".").collect();
                let prereleases = split.into_iter().map(|s| s.to_owned()).collect();
                Some(prereleases)
            }
            None => None,
        };

        Version {
            major,
            minor,
            patch,
            prerelease,
            empty: false,
            any: false,
        }
    }

    pub fn is_any(&self) -> bool {
        self.any
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn has_prerelease(&self) -> bool {
        match self.prerelease {
            Some(ref pre) => pre.len() > 0,
            None => false,
        }
    }

    fn compare_main(&self, other: &Self) -> Ordering {
        let mut compare_result = self.major.cmp(&other.major);
        if let Ordering::Equal = compare_result {
            compare_result = self.minor.cmp(&other.minor);
            if let Ordering::Equal = compare_result {
                compare_result = self.patch.cmp(&other.patch)
            }
        }

        compare_result
    }

    fn compare_pre(&self, other: &Self) -> Ordering {
        //TODO: chain with .then
        if self.prerelease.is_some() && other.prerelease.is_none() {
            Ordering::Less
        } else if self.prerelease.is_none() && other.prerelease.is_some() {
            Ordering::Greater
        } else if self.prerelease.is_none() && other.prerelease.is_none() {
            Ordering::Equal
        } else {
            let mut self_prerelease = self.prerelease.clone().unwrap().into_iter();
            let mut other_prerelease = other.prerelease.clone().unwrap().into_iter();
            loop {
                let a = self_prerelease.next();
                let b = other_prerelease.next();

                if a.is_none() && b.is_none() {
                    return Ordering::Equal;
                } else if b.is_none() {
                    return Ordering::Greater;
                } else if a.is_none() {
                    return Ordering::Less;
                } else if a.clone().unwrap().eq(&b.clone().unwrap()) {
                    continue;
                } else {
                    return compare_identifiers(a.clone().unwrap(), b.clone().unwrap());
                }
            }
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            Ok(())
        } else {
            let mut fmt = format!("{}.{}.{}", self.major, self.minor, self.patch);
            if let Some(ref prerelease) = self.prerelease {
                fmt = format!("{}-{}", fmt, prerelease.join("."))
            }

            write!(f, "{}", fmt)
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Equal
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        let res = self.compare_main(other);
        Some(match res {
            Ordering::Equal => self.compare_pre(other),
            _ => res,
        })
    }
}
