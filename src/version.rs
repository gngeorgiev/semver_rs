use crate::builder::{Builder, Options, Parseable};
use crate::error::Error;
use crate::expressions::{VERSION, VERSION_LOOSE};
use crate::util::compare_identifiers;
use std::hash::Hash;

use std::{cmp::Ordering, fmt, str};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `version` is described by the `v2.0.0` specification found at [semver](https://semver.org/).
///
/// A leading `=` or `v` character is stripped off and ignored.
#[derive(Default, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Version {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
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
            let prerelease = cap.get(4).map(|v| v.as_str().to_owned());
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

    /// Constructs a version from its already parsed parts, e.g. `Version::from_parts(1, 2, 3, None)`.
    pub fn from_parts(major: i64, minor: i64, patch: i64, prerelease: Option<String>) -> Self {
        let prerelease = match prerelease {
            Some(pre) => pre
                .split('.')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
                .into(),
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
            Some(ref pre) => !pre.is_empty(),
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
        match (self.prerelease.as_ref(), other.prerelease.as_ref()) {
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
            (Some(pre1), Some(pre2)) => {
                let mut pre1 = pre1.iter();
                let mut pre2 = pre2.iter();
                loop {
                    match (pre1.next().as_ref(), pre2.next().as_ref()) {
                        (Some(a), Some(b)) => match a.eq(b) {
                            true => continue,
                            false => return compare_identifiers(a, b),
                        },
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                    }
                }
            }
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_empty() {
            let fmt = if let Some(ref prerelease) = self.prerelease {
                format!(
                    "{}.{}.{}-{}",
                    self.major,
                    self.minor,
                    self.patch,
                    prerelease.join(".")
                )
            } else {
                format!("{}.{}.{}", self.major, self.minor, self.patch)
            };

            write!(f, "{}", fmt)?;
        }

        Ok(())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        let res = self.compare_main(other);
        match res {
            Ordering::Equal => self.compare_pre(other),
            _ => res,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_compare<U, V>(va: &[U], vb: &[V]) -> bool
    where
        U: AsRef<str>,
        V: AsRef<str>,
    {
        (va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| a.as_ref() == b.as_ref())
    }
    #[test]
    fn test_sort() {
        // Create a vector of semver_rs::Version
        let mut input_versions_list = vec![
            "1.2.3-dev",
            "1.2.3-dev.1",
            "1.2.3-dev.cache",
            "1.2.3",
            "2.0.0",
            "1.1.1",
        ]
        .iter()
        .flat_map(|version| Version::new(version).parse())
        .collect::<Vec<Version>>();
        // Sort it inplace
        input_versions_list.sort();
        // Create the expected Vec<&str>
        let output: Vec<String> = input_versions_list
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let expected = vec![
            "1.1.1",
            "1.2.3-dev",
            "1.2.3-dev.1",
            "1.2.3-dev.cache",
            "1.2.3",
            "2.0.0",
        ];
        assert!(vec_compare(&output, &expected));
    }
}
