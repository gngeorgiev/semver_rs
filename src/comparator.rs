use crate::builder::Options;
use crate::error::Error;
use crate::expressions::{
    COMPARATOR, COMPARATOR_LOOSE, COMP_REPLACE_CARETS, COMP_REPLACE_CARETS_LOOSE,
    COMP_REPLACE_STARS, COMP_REPLACE_TILDES, COMP_REPLACE_TILDES_LOOSE, COMP_REPLACE_XRANGES,
    COMP_REPLACE_XRANGES_LOOSE,
};
use crate::operator::Operator;
use crate::util::{get_prerelease_prefix, increment_version, is_any_version, replacer};
use crate::version::Version;

use std::fmt;
use std::{borrow::Cow, cmp::Ordering};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ComparatorPair is a simple struct that can hold two comparators
/// it knows how to format its Comparators
#[derive(Debug)]
pub(crate) struct ComparatorPair(pub Option<Comparator>, pub Option<Comparator>);

impl fmt::Display for ComparatorPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.0.as_ref(), self.1.as_ref()) {
            (Some(c1), Some(c2)) => write!(f, "{} {}", c1, c2),
            (Some(c1), None) => write!(f, "{}", c1),
            (None, Some(c2)) => write!(f, "{}", c2),
            (None, None) => Ok(()),
        }
    }
}

/// A `Comparator` is composed of an [Operator](crate::operator::Operator) and a [Version](create::version::Version).
/// Comparators are the building blocks of [Range](crate::range::Range)s
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Comparator {
    pub operator: Operator,
    pub version: Version,

    empty: bool,
}

impl Comparator {
    pub fn empty() -> Self {
        Comparator {
            operator: Operator::Empty,
            version: Version::any(), //empty comparator matches any version
            empty: true,
        }
    }

    pub fn from_parts(operator: Operator, version: Version) -> Self {
        Comparator {
            operator,
            version,
            empty: false,
        }
    }

    pub fn new(comp: &str, opts: Option<Options>) -> Result<Self, Error> {
        let cap = match opts.unwrap_or_default().loose {
            true => COMPARATOR_LOOSE.captures(comp),
            false => COMPARATOR.captures(comp),
        };
        let cap = match cap {
            Some(cap) => cap,
            None => return Err(Error::InvalidComparator(comp.into())),
        };

        let operator = match cap.get(1) {
            Some(op) => {
                let op = Operator::new(op.as_str());
                if op == Operator::Eq || op == Operator::StrictEq {
                    Operator::Empty
                } else {
                    op
                }
            }
            None => Operator::Empty,
        };

        let version = if cap.get(2).is_none() {
            Version::any()
        } else {
            let major = cap.get(3).map_or("", |v| v.as_str());
            let minor = cap.get(4).map_or("", |v| v.as_str());
            let patch = cap.get(5).map_or("", |v| v.as_str());
            let prerelease = cap.get(6).map(|v| v.as_str().to_owned());
            Version::from_parts(major.parse()?, minor.parse()?, patch.parse()?, prerelease)
        };

        Ok(Comparator {
            operator,
            version,
            empty: false,
        })
    }

    pub fn normalize(input: &str, loose: bool) -> String {
        // TODO: Can we avoid using to_owned for each comparator function?
        let mut comp = Comparator::replace_carets(input, loose).as_ref().to_owned();

        comp = Comparator::replace_tildes(&comp, loose).as_ref().to_owned();

        comp = Comparator::replace_xranges(&comp, loose)
            .as_ref()
            .to_owned();

        Comparator::replace_stars(&comp).as_ref().to_owned()
    }

    fn replace_stars(comp: &str) -> Cow<'_, str> {
        COMP_REPLACE_STARS.replace_all(comp, "")
    }

    fn replace_xranges(comp: &str, loose: bool) -> Cow<'_, str> {
        let repl = replacer(|args: &[String]| {
            let version = args[0].as_str();
            let mut op = args[1].as_str();
            let major = args[2].as_str();
            let minor = args[3].as_str();
            let patch = args[4].as_str();

            let is_any_major = is_any_version(major);
            let is_any_minor = is_any_major || is_any_version(minor);
            let is_any_patch = is_any_minor || is_any_version(patch);
            let is_any_version = is_any_patch;

            if op == "=" && is_any_version {
                op = ""
            }

            let mut op = Operator::new(op);

            if is_any_major {
                if op == Operator::Lt || op == Operator::Gt {
                    Cow::Borrowed("<0.0.0")
                } else {
                    Cow::Borrowed("*")
                }
            } else if op != Operator::Empty && is_any_version {
                let mut parsed_minor = 0;
                let mut parsed_major = major.parse::<usize>().unwrap();
                let mut parsed_patch = patch;
                if !is_any_minor {
                    parsed_minor = minor.parse::<usize>().unwrap();
                }
                if is_any_patch {
                    parsed_patch = "0"
                }

                if op == Operator::Gt {
                    op = Operator::Gte;
                    if is_any_minor {
                        parsed_major = increment_version(major);
                        parsed_minor = 0;
                        parsed_patch = "0"
                    } else if is_any_patch {
                        parsed_minor = increment_version(minor);
                        parsed_patch = "0";
                    }
                } else if op == Operator::Lte {
                    op = Operator::Lt;
                    if is_any_minor {
                        parsed_major = increment_version(major);
                    } else {
                        parsed_minor = increment_version(minor);
                    }
                }

                Cow::Owned(format!(
                    "{}{}.{}.{}",
                    op, parsed_major, parsed_minor, parsed_patch
                ))
            } else if is_any_minor {
                Cow::Owned(format!(
                    "{}{}.0.0 {}{}.0.0",
                    Operator::Gte,
                    major,
                    Operator::Lt,
                    increment_version(major)
                ))
            } else if is_any_patch {
                Cow::Owned(format!(
                    "{}{}.{}.0 {}{}.{}.0",
                    Operator::Gte,
                    major,
                    minor,
                    Operator::Lt,
                    major,
                    increment_version(minor)
                ))
            } else {
                // TODO: we might be able to get a reference to this
                Cow::Owned(version.to_owned())
            }
        });

        match loose {
            true => COMP_REPLACE_XRANGES_LOOSE.replace_all(comp, repl),
            false => COMP_REPLACE_XRANGES.replace_all(comp, repl),
        }
    }

    fn replace_tildes(comp: &str, loose: bool) -> Cow<'_, str> {
        //TODO: not yet sure why this workaround is needed
        if comp == "~" {
            return Cow::Borrowed("*");
        }

        let repl = replacer(|args: &[String]| {
            let major = args[1].as_str();
            let minor = args[2].as_str();
            let patch = args[3].as_str();
            let prerelease = args[4].as_str();

            if is_any_version(major) {
                Cow::Borrowed("")
            } else if is_any_version(minor) {
                Cow::Owned(format!(
                    "{}{}.0.0 {}{}.0.0",
                    Operator::Gte,
                    major,
                    Operator::Lt,
                    increment_version(major)
                ))
            } else if is_any_version(patch) {
                //'>=' + M + '.' + m + '.0 <' + M + '.' + (+m + 1) + '.0';
                Cow::Owned(format!(
                    "{}{}.{}.0 {}{}.{}.0",
                    Operator::Gte,
                    major,
                    minor,
                    Operator::Lt,
                    major,
                    increment_version(minor)
                ))
            } else if !prerelease.is_empty() {
                Cow::Owned(format!(
                    "{}{}.{}.{}{}{} {}{}.{}.0",
                    Operator::Gte,
                    major,
                    minor,
                    patch,
                    get_prerelease_prefix(prerelease),
                    prerelease,
                    Operator::Lt,
                    major,
                    increment_version(minor)
                ))
            } else {
                Cow::Owned(format!(
                    "{}{}.{}.{} {}{}.{}.0",
                    Operator::Gte,
                    major,
                    minor,
                    patch,
                    Operator::Lt,
                    major,
                    increment_version(minor)
                ))
            }
        });

        match loose {
            true => COMP_REPLACE_TILDES_LOOSE.replace_all(comp, repl),
            false => COMP_REPLACE_TILDES.replace_all(comp, repl),
        }
    }

    fn replace_carets(comp: &str, loose: bool) -> Cow<'_, str> {
        if comp == "^" {
            //TODO: not yet sure why this workaround is needed
            return Cow::Borrowed("*");
        }

        let repl = replacer(|args: &[String]| {
            let major = args[1].as_str();
            let minor = args[2].as_str();
            let patch = args[3].as_str();
            let prerelease = args[4].as_str();

            if is_any_version(major) {
                Cow::Borrowed("")
            } else if is_any_version(minor) {
                Cow::Owned(format!(">={}.0.0 <{}.0.0", major, increment_version(major)))
            } else if is_any_version(patch) {
                if major == "0" {
                    Cow::Owned(format!(
                        ">={}.{}.0 <{}.{}.0",
                        major,
                        minor,
                        major,
                        increment_version(minor)
                    ))
                } else {
                    Cow::Owned(format!(
                        ">={}.{}.0 <{}.0.0",
                        major,
                        minor,
                        increment_version(major),
                    ))
                }
            } else if !prerelease.is_empty() {
                if major == "0" {
                    if minor == "0" {
                        Cow::Owned(format!(
                            ">= {}.{}.{}{}{} <{}.{}.{}",
                            major,
                            minor,
                            patch,
                            get_prerelease_prefix(prerelease),
                            prerelease,
                            major,
                            minor,
                            increment_version(patch)
                        ))
                    } else {
                        Cow::Owned(format!(
                            ">= {}.{}.{}{} <{}.{}.0",
                            major,
                            minor,
                            patch,
                            prerelease,
                            major,
                            increment_version(minor)
                        ))
                    }
                } else {
                    Cow::Owned(format!(
                        ">={}.{}.{}{} <{}.0.0",
                        major,
                        minor,
                        patch,
                        prerelease,
                        increment_version(major)
                    ))
                }
            } else if major == "0" {
                if minor == "0" {
                    Cow::Owned(format!(
                        ">={}.{}.{} <{}.{}.{}",
                        major,
                        minor,
                        patch,
                        major,
                        minor,
                        increment_version(patch),
                    ))
                } else {
                    Cow::Owned(format!(
                        "=>{}.{}.{} <{}.{}.0",
                        major,
                        minor,
                        patch,
                        major,
                        increment_version(minor),
                    ))
                }
            } else {
                Cow::Owned(format!(
                    ">={}.{}.{} <{}.0.0",
                    major,
                    minor,
                    patch,
                    increment_version(major),
                ))
            }
        });

        match loose {
            true => COMP_REPLACE_CARETS_LOOSE.replace_all(comp, repl),
            false => COMP_REPLACE_CARETS.replace_all(comp, repl),
        }
    }

    pub fn test(&self, version: &Version) -> bool {
        if self.version.is_any() {
            true
        } else if self.version.is_empty() {
            false
        } else {
            Self::cmp_versions(version, &self.operator, &self.version)
        }
    }

    //this is the same as the cmp fn in compare_fns, but implemented for instances of Version
    fn cmp_versions(a: &Version, op: &Operator, b: &Version) -> bool {
        match op {
            Operator::Eq | Operator::StrictEq | Operator::Empty => {
                a.partial_cmp(b).unwrap() == Ordering::Equal
            }
            Operator::Ne | Operator::StrictNe => a.partial_cmp(b).unwrap() != Ordering::Equal,
            Operator::Gt => a.partial_cmp(b).unwrap() == Ordering::Greater,
            Operator::Gte => {
                let ord = a.partial_cmp(b).unwrap();
                ord == Ordering::Greater || ord == Ordering::Equal
            }
            Operator::Lt => a.partial_cmp(b).unwrap() == Ordering::Less,
            Operator::Lte => {
                let ord = a.partial_cmp(b).unwrap();
                ord == Ordering::Less || ord == Ordering::Equal
            }
        }
    }
}

impl fmt::Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.empty {
            write!(f, "{}{}", self.operator, self.version)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_carets() {
        let v = vec![
            ("^1.2.3", ">=1.2.3 <2.0.0"),
            ("^1.2.0", ">=1.2.0 <2.0.0"),
            ("^1.2", ">=1.2.0 <2.0.0"),
            ("^2.0", ">=2.0.0 <3.0.0"),
            ("^2", ">=2.0.0 <3.0.0"),
            ("^", "*"),
        ];
        for (input, output) in v {
            let res = Comparator::replace_carets(input, false);
            assert_eq!(output, res);
        }
    }

    #[test]
    fn replace_tildes() {
        let v = vec![
            ("~2", ">=2.0.0 <3.0.0"),
            ("~2.0", ">=2.0.0 <2.1.0"),
            ("~1.2", ">=1.2.0 <1.3.0"),
            ("~1.2.3", ">=1.2.3 <1.3.0"),
            ("~1.2.0", ">=1.2.0 <1.3.0"),
            ("~", "*"),
        ];
        for (input, output) in v {
            let res = Comparator::replace_tildes(input, false);
            assert_eq!(output, res);
        }
    }

    #[test]
    fn replace_xranges() {
        let v = vec![
            (">1", ">=2.0.0"),
            (">1.2", ">=1.3.0"),
            ("<=0.7.x", "<0.8.0"),
            ("<=7.x", "<8.0.0"),
        ];

        for (input, output) in v {
            let res = Comparator::replace_xranges(input, false);
            assert_eq!(output, res);
        }
    }

    #[test]
    fn replace_stars() {
        let v = vec![("*", "")];
        for (input, output) in v {
            let res = Comparator::replace_stars(input);
            assert_eq!(output, res);
        }
    }
}
