use crate::builder::Options;
use crate::error::{Error, ErrorKind};
use crate::expressions::{
    COMPARATOR, COMPARATOR_LOOSE, COMP_REPLACE_CARETS, COMP_REPLACE_CARETS_LOOSE,
    COMP_REPLACE_STARS, COMP_REPLACE_TILDES, COMP_REPLACE_TILDES_LOOSE, COMP_REPLACE_XRANGES,
    COMP_REPLACE_XRANGES_LOOSE,
};
use crate::operator::Operator;
use crate::util::{ensure_prerelease_dash, increment_version, is_any_version, replacer};
use crate::version::Version;

use std::cmp::Ordering;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ComparatorPair is a simple struct that can hold two comparators
/// it knows how to format its Comparators
#[derive(Debug)]
pub(crate) struct ComparatorPair(pub Option<Comparator>, pub Option<Comparator>);

impl fmt::Display for ComparatorPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_some() && self.1.is_none() {
            write!(f, "{}", self.0.clone().unwrap())
        } else if self.0.is_some() && self.1.is_some() {
            write!(f, "{} {}", self.0.clone().unwrap(), self.1.clone().unwrap())
        } else {
            Ok(())
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

    pub fn new(comp: String, opts: Option<Options>) -> Result<Self, Error> {
        let cap = match opts.unwrap_or_default().loose {
            true => COMPARATOR_LOOSE.captures(&comp),
            false => COMPARATOR.captures(&comp),
        };
        let cap = match cap {
            Some(cap) => cap,
            None => return Err(Error::new(ErrorKind::InvalidComparator(comp.clone()))),
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
            let major = cap.get(3).map_or("", |v| v.as_str()).to_owned();
            let minor = cap.get(4).map_or("", |v| v.as_str()).to_owned();
            let patch = cap.get(5).map_or("", |v| v.as_str()).to_owned();
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
        let mut comp = String::from(input);
        comp = Comparator::replace_carets(&comp, loose);
        comp = Comparator::replace_tildes(&comp, loose);
        comp = Comparator::replace_xranges(&comp, loose);
        comp = Comparator::replace_stars(&comp);
        comp
    }

    fn replace_stars(comp: &str) -> String {
        COMP_REPLACE_STARS.replace_all(comp, "").to_string()
    }

    fn replace_xranges(comp: &str, loose: bool) -> String {
        let repl = replacer(|args: Vec<String>| {
            let version = args[0].clone();
            let mut op = args[1].clone();
            let mut major = args[2].clone();
            let mut minor = args[3].clone();
            let mut patch = args[4].clone();

            let is_any_major = is_any_version(&major);
            let is_any_minor = is_any_major || is_any_version(&minor);
            let is_any_patch = is_any_minor || is_any_version(&patch);
            let is_any_version = is_any_patch;

            if op == "=" && is_any_version {
                op = String::new();
            }

            let mut op = Operator::new(&op);

            if is_any_major {
                if op == Operator::Lt || op == Operator::Gt {
                    String::from("<0.0.0")
                } else {
                    String::from("*")
                }
            } else if op != Operator::Empty && is_any_version {
                if is_any_minor {
                    minor = String::from("0");
                }
                if is_any_patch {
                    patch = String::from("0");
                }

                if op == Operator::Gt {
                    op = Operator::Gte;
                    if is_any_minor {
                        major = increment_version(&major);
                        minor = String::from("0");
                        patch = String::from("0");
                    } else if is_any_patch {
                        minor = increment_version(&minor);
                        patch = String::from("0");
                    }
                } else if op == Operator::Lte {
                    op = Operator::Lt;
                    if is_any_minor {
                        major = increment_version(&major);
                    } else {
                        minor = increment_version(&minor);
                    }
                }

                format!("{}{}.{}.{}", op, major, minor, patch)
            } else if is_any_minor {
                format!(
                    "{}{}.0.0 {}{}.0.0",
                    Operator::Gte,
                    major,
                    Operator::Lt,
                    increment_version(&major)
                )
            } else if is_any_patch {
                format!(
                    "{}{}.{}.0 {}{}.{}.0",
                    Operator::Gte,
                    major,
                    minor,
                    Operator::Lt,
                    major,
                    increment_version(&minor)
                )
            } else {
                version
            }
        });

        match loose {
            true => COMP_REPLACE_XRANGES_LOOSE.replace_all(comp, repl),
            false => COMP_REPLACE_XRANGES.replace_all(comp, repl),
        }
        .to_string()
    }

    fn replace_tildes(comp: &str, loose: bool) -> String {
        //TODO: not yet sure why this workaround is needed
        if comp == "~" {
            return String::from("*");
        }

        let repl = replacer(|args: Vec<String>| {
            let major = args[1].clone();
            let minor = args[2].clone();
            let patch = args[3].clone();
            let prerelease = args[4].clone();

            if is_any_version(&major) {
                String::new()
            } else if is_any_version(&minor) {
                format!(
                    "{}{}.0.0 {}{}.0.0",
                    Operator::Gte,
                    major,
                    Operator::Lt,
                    increment_version(&major)
                )
            } else if is_any_version(&patch) {
                //'>=' + M + '.' + m + '.0 <' + M + '.' + (+m + 1) + '.0';
                format!(
                    "{}{}.{}.0 {}{}.{}.0",
                    Operator::Gte,
                    major,
                    minor,
                    Operator::Lt,
                    major,
                    increment_version(&minor)
                )
            } else if !prerelease.is_empty() {
                let prerelease = ensure_prerelease_dash(&prerelease);
                format!(
                    "{}{}.{}.{}{} {}{}.{}.0",
                    Operator::Gte,
                    major,
                    minor,
                    patch,
                    prerelease,
                    Operator::Lt,
                    major,
                    increment_version(&minor)
                )
            } else {
                format!(
                    "{}{}.{}.{} {}{}.{}.0",
                    Operator::Gte,
                    major,
                    minor,
                    patch,
                    Operator::Lt,
                    major,
                    increment_version(&minor)
                )
            }
        });

        match loose {
            true => COMP_REPLACE_TILDES_LOOSE.replace_all(comp, repl),
            false => COMP_REPLACE_TILDES.replace_all(comp, repl),
        }
        .to_string()
    }

    fn replace_carets(comp: &str, loose: bool) -> String {
        if comp == "^" {
            //TODO: not yet sure why this workaround is needed
            return String::from("*");
        }

        let repl = replacer(|args: Vec<String>| {
            let major = args[1].clone();
            let minor = args[2].clone();
            let patch = args[3].clone();
            let prerelease = args[4].clone();

            if is_any_version(&major) {
                String::new()
            } else if is_any_version(&minor) {
                format!(">={}.0.0 <{}.0.0", major, increment_version(&major))
            } else if is_any_version(&patch) {
                if major == "0" {
                    format!(
                        ">={}.{}.0 <{}.{}.0",
                        major,
                        minor,
                        major,
                        increment_version(&minor)
                    )
                } else {
                    format!(">={}.{}.0 <{}.0.0", major, minor, increment_version(&major),)
                }
            } else if !prerelease.is_empty() {
                let prerelease = ensure_prerelease_dash(&prerelease);
                if major == "0" {
                    if minor == "0" {
                        format!(
                            ">= {}.{}.{}{} <{}.{}.{}",
                            major,
                            minor,
                            patch,
                            prerelease,
                            major,
                            minor,
                            increment_version(&patch)
                        )
                    } else {
                        format!(
                            ">= {}.{}.{}{} <{}.{}.0",
                            major,
                            minor,
                            patch,
                            prerelease,
                            major,
                            increment_version(&minor)
                        )
                    }
                } else {
                    format!(
                        ">={}.{}.{}{} <{}.0.0",
                        major,
                        minor,
                        patch,
                        prerelease,
                        increment_version(&major)
                    )
                }
            } else if major == "0" {
                if minor == "0" {
                    format!(
                        ">={}.{}.{} <{}.{}.{}",
                        major,
                        minor,
                        patch,
                        major,
                        minor,
                        increment_version(&patch),
                    )
                } else {
                    format!(
                        "=>{}.{}.{} <{}.{}.0",
                        major,
                        minor,
                        patch,
                        major,
                        increment_version(&minor),
                    )
                }
            } else {
                format!(
                    ">={}.{}.{} <{}.0.0",
                    major,
                    minor,
                    patch,
                    increment_version(&major),
                )
            }
        });

        match loose {
            true => COMP_REPLACE_CARETS_LOOSE.replace_all(comp, repl),
            false => COMP_REPLACE_CARETS.replace_all(comp, repl),
        }
        .to_string()
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
