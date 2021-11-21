use crate::builder::{Builder, Options, Parseable};
use crate::comparator::{Comparator, ComparatorPair};
use crate::error::{Error, ErrorKind};
use crate::expressions::{
    COMPARATOR_LOOSE, COMP_REPLACE_CARETS, RANGE_HYPHEN, RANGE_HYPHEN_LOOSE, RANGE_OR,
    RANGE_TRIM_CARET, RANGE_TRIM_OPERATORS, RANGE_TRIM_TILDE, SPLIT_SPACES,
};
use crate::operator::Operator;
use crate::util::{is_any_version, match_at_index_str};
use crate::version::Version;
use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `version range` is a set of `comparators` which specify versions that satisfy the `range`.
/// A comparator is composed of an operator and a version. The set of primitive operators is:
///
/// `<` Less than
///
/// `<=` Less than or equal to
///
/// `>` Greater than
///
/// `>=` Greater than or equal to
///
/// `=` Equal. If no operator is specified, then equality is assumed, so this operator is optional, but MAY be included.
///
///
/// For example, the comparator `>=1.2.7` would match the versions `1.2.7`, `1.2.8`, `2.5.3`, and `1.3.9`, but not the versions `1.2.6` or `1.1.0`.
///
/// Comparators can be joined by whitespace to form a comparator set, which is satisfied by the intersection of all of the comparators it includes.
///
/// A range is composed of one or more comparator sets, joined by ||. A version matches a range if and only if every comparator in at least one of the ||-separated comparator sets is satisfied by the version.
///
/// For example, the range `>=1.2.7 <1.3.0` would match the versions `1.2.7`, `1.2.8`, and `1.2.99`, but not the versions `1.2.6`, `1.3.0`, or `1.1.0`.
///
/// The range `1.2.7 || >=1.2.9 <2.0.0` would match the versions `1.2.7`, `1.2.9`, and `1.4.6`, but not the versions `1.2.8` or `2.0.0`.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Range {
    pub(crate) comparators: Vec<Vec<Comparator>>,

    opts: Option<Options>,
}

impl<'p> Parseable<'p> for Range {
    fn parse(range_input: &'p str, opts: Option<Options>) -> Result<Self, Error> {
        let loose = opts.clone().unwrap_or_default().loose;

        if range_input.is_empty() {
            let comp = Comparator::empty();
            return Ok(Range {
                comparators: vec![vec![comp]],
                opts,
            });
        }

        let comparators_opts = opts.clone();
        let comparators_result: Result<Vec<Option<Vec<Comparator>>>, Error> = RANGE_OR
            .split(range_input)
            .map(move |range: &str| {
                //1. trim the range
                let range = range.trim();

                //2. replace hyphens `1.2.3 - 1.2.4` => `>=1.2.3 <=1.2.4`
                let range = if let Some(range) = Range::replace_hyphens(range, loose)? {
                    range.to_string()
                } else if let Some(range) = Range::replace_carets(range)? {
                    range.to_string()
                } else {
                    //3. trim the spaces around operators `> 1.2.3 < 1.2.5` => `>1.2.3 <1.2.5`
                    let range = Range::trim_operators(range);

                    //4. trim spaces around the tilde operator `~ 1.2.3` => `~1.2.3`
                    let range = Range::trim_tilde(&range);

                    //5. trim spaces around the caret operator `^ 1.2.3` => `^1.2.3`
                    let range = Range::trim_caret(&range);

                    //6. trim all the spaces that are left `1.2.3  1.2.4` => `1.2.3 1.2.4`
                    let range = Range::trim_spaces(&range);

                    range.to_string()
                };

                let comparators_parsed: Vec<String> = range
                    .split(' ')
                    .map(|c| Comparator::normalize(c, loose))
                    .collect::<Vec<_>>();

                let comparators_parsed = comparators_parsed.join(" ");
                if comparators_parsed.is_empty() {
                    let comp = Comparator::empty();
                    return Ok(Some(vec![comp]));
                }

                // TODO: this split should yield an array with one empty string inside
                // when used on an empty string, just like in the original npm package.
                // The condition above is a workaround atm
                

                let opts = comparators_opts.clone();
                let comparators = SPLIT_SPACES.split(&comparators_parsed)
                    .filter(|c| {
                        if loose {
                            COMPARATOR_LOOSE.is_match(c)
                        } else {
                            true
                        }
                    })
                    .map(move |r| Comparator::new(r.to_owned(), opts.clone()))
                    .collect::<Result<Vec<_>, Error>>();

                match comparators {
                    Ok(comp) => {
                        if !comp.is_empty() {
                            Ok(Some(comp))
                        } else {
                            Ok(None)
                        }
                    }
                    Err(err) => Err(err),
                }
            })
            .collect();

        let comparators: Vec<Vec<Comparator>> =
            comparators_result?.into_iter().flatten().collect();

        if comparators.is_empty() {
            Err(Error::new(ErrorKind::InvalidRange(range_input.to_owned())))
        } else {
            Ok(Range { comparators, opts })
        }
    }
}

impl<'p> Range {
    /// Construct a new Range, e.g. `>=1.2.4`.
    pub fn new(range: &'p str) -> Builder<'p, Self> {
        Builder::new(range)
    }

    fn trim_spaces(range: &str) -> Cow<str> {
        //the other regexes won't allocate if they don't match, however this one will always allocate
        //so we check whether there's a match
        if SPLIT_SPACES.is_match(range) {
            //avoid collecting to not allocate an intermediate vec
            let mut buf = String::new();
            SPLIT_SPACES.split(range).for_each(|s| {
                buf.push_str(s);
                buf.push(' ');
            });
            buf.pop();

            Cow::Owned(buf)
        } else {
            Cow::Borrowed(range)
        }
    }

    fn trim_caret(range: &str) -> Cow<str> {
        RANGE_TRIM_CARET.replace_all(range, "$1^")
    }

    fn trim_tilde(range: &str) -> Cow<str> {
        RANGE_TRIM_TILDE.replace_all(range, "$1~")
    }

    fn trim_operators(range: &str) -> Cow<str> {
        RANGE_TRIM_OPERATORS.replace_all(range, "$1$2$3")
    }

    fn replace_hyphens(range: &str, loose: bool) -> Result<Option<ComparatorPair>, Error> {
        let mut caps = match loose {
            true => RANGE_HYPHEN_LOOSE.captures_iter(range),
            false => RANGE_HYPHEN.captures_iter(range),
        };
        let cap = match caps.next() {
            Some(cap) => cap,
            None => return Ok(None),
        };

        let from = match_at_index_str(&cap, 1);
        let from_major = match_at_index_str(&cap, 2);
        let from_minor = match_at_index_str(&cap, 3);
        let from_patch = match_at_index_str(&cap, 4);

        let comparator_from = if is_any_version(from_major) {
            Comparator::empty()
        } else if is_any_version(from_minor) {
            Comparator::from_parts(
                Operator::Gte,
                Version::from_parts(from_major.parse()?, 0, 0, None),
            )
        } else if is_any_version(from_patch) {
            Comparator::from_parts(
                Operator::Gte,
                Version::from_parts(from_major.parse()?, from_minor.parse()?, 0, None),
            )
        } else {
            Comparator::from_parts(Operator::Gte, Version::new(from).parse()?)
        };

        let to = match_at_index_str(&cap, 7);
        let to_major = match_at_index_str(&cap, 8);
        let to_minor = match_at_index_str(&cap, 9);
        let to_patch = match_at_index_str(&cap, 10);
        let to_prerelease = match_at_index_str(&cap, 11);

        let comparator_to = if is_any_version(to_major) {
            Comparator::empty()
        } else if is_any_version(to_minor) {
            let mut to_major = to_major.parse()?;
            to_major += 1;

            Comparator::from_parts(Operator::Lt, Version::from_parts(to_major, 0, 0, None))
        } else if is_any_version(to_patch) {
            let mut to_minor = to_minor.parse()?;
            to_minor += 1;
            Comparator::from_parts(
                Operator::Lt,
                Version::from_parts(to_major.parse()?, to_minor, 0, None),
            )
        } else if !to_prerelease.is_empty() {
            Comparator::from_parts(
                Operator::Lte,
                Version::from_parts(
                    to_major.parse()?,
                    to_minor.parse()?,
                    to_patch.parse()?,
                    Some(to_prerelease.to_string()),
                ),
            )
        } else {
            Comparator::from_parts(Operator::Lte, Version::new(to).parse()?)
        };

        Ok(Some(ComparatorPair(
            Some(comparator_from),
            Some(comparator_to),
        )))
    }

    fn replace_carets(range: &str) -> Result<Option<ComparatorPair>, Error> {
        let mut caps = COMP_REPLACE_CARETS.captures_iter(range);
        let cap = match caps.next() {
            Some(cap) => cap,
            None => return Ok(None),
        };

        let major = match_at_index_str(&cap, 1);
        let minor = match_at_index_str(&cap, 2);
        let patch = match_at_index_str(&cap, 3);
        let prerelease = match_at_index_str(&cap, 4);

        let mut cmp = ComparatorPair(None, None);
        if is_any_version(major) {
            cmp.0 = Some(Comparator::empty());
        } else if is_any_version(minor) {
            let major = major.parse()?;
            cmp.0 = Some(Comparator::from_parts(
                Operator::Gte,
                Version::from_parts(major, 0, 0, None),
            ));
            cmp.1 = Some(Comparator::from_parts(
                Operator::Lt,
                Version::from_parts(major + 1, 0, 0, None),
            ));
        } else if is_any_version(patch) {
            let major = major.parse()?;
            let minor = minor.parse()?;
            if major == 0 {
                cmp.0 = Some(Comparator::from_parts(
                    Operator::Gte,
                    Version::from_parts(major, minor, 0, None),
                ));
                cmp.1 = Some(Comparator::from_parts(
                    Operator::Lt,
                    Version::from_parts(major, minor + 1, 0, None),
                ));
            } else {
                cmp.0 = Some(Comparator::from_parts(
                    Operator::Gte,
                    Version::from_parts(major, minor, 0, None),
                ));
                cmp.1 = Some(Comparator::from_parts(
                    Operator::Lt,
                    Version::from_parts(major + 1, 0, 0, None),
                ));
            }
        } else if !prerelease.is_empty() {
            //this unwrap will never panic since we already verified that we have at least
            //one char in the string
            let prerelease = if prerelease.starts_with('-') {
                prerelease.to_string()
            } else {
                format!("-{}", prerelease)
            };

            let major = major.parse()?;
            let minor = minor.parse()?;
            let patch = patch.parse()?;

            if major == 0 {
                if minor == 0 {
                    cmp.0 = Some(Comparator::from_parts(
                        Operator::Gte,
                        Version::from_parts(major, minor, patch, Some(prerelease)),
                    ));
                    cmp.1 = Some(Comparator::from_parts(
                        Operator::Lt,
                        Version::from_parts(major, minor, patch + 1, None),
                    ));
                } else {
                    cmp.0 = Some(Comparator::from_parts(
                        Operator::Gte,
                        Version::from_parts(major, minor, patch, Some(prerelease)),
                    ));
                    cmp.1 = Some(Comparator::from_parts(
                        Operator::Lt,
                        Version::from_parts(major, minor + 1, 0, None),
                    ));
                }
            } else {
                cmp.0 = Some(Comparator::from_parts(
                    Operator::Gte,
                    Version::from_parts(major, minor, patch, Some(prerelease)),
                ));
                cmp.1 = Some(Comparator::from_parts(
                    Operator::Lt,
                    Version::from_parts(major + 1, 0, 0, None),
                ));
            }
        } else {
            let major = major.parse()?;
            let minor = minor.parse()?;
            let patch = patch.parse()?;

            if major == 0 {
                if minor == 0 {
                    cmp.0 = Some(Comparator::from_parts(
                        Operator::Gte,
                        Version::from_parts(major, minor, patch, None),
                    ));
                    cmp.1 = Some(Comparator::from_parts(
                        Operator::Lt,
                        Version::from_parts(major, minor, patch + 1, None),
                    ));
                } else {
                    cmp.0 = Some(Comparator::from_parts(
                        Operator::Gte,
                        Version::from_parts(major, minor, patch, None),
                    ));
                    cmp.1 = Some(Comparator::from_parts(
                        Operator::Lt,
                        Version::from_parts(major, minor + 1, 0, None),
                    ));
                }
            } else {
                cmp.0 = Some(Comparator::from_parts(
                    Operator::Gte,
                    Version::from_parts(major, minor, patch, None),
                ));
                cmp.1 = Some(Comparator::from_parts(
                    Operator::Lt,
                    Version::from_parts(major + 1, 0, 0, None),
                ));
            }
        }

        Ok(Some(cmp))
    }

    /// Tests whether a `version` is in this `range`.
    pub fn test(&self, version: &Version) -> bool {
        let include_prerelease = match self.opts {
            Some(ref opts) => opts.include_prerelease,
            None => false,
        };

        self.comparators
            .iter()
            .find(move |comparators| {
                for c in comparators.iter() {
                    if !c.test(version) {
                        return false;
                    }
                }

                if version.has_prerelease() && !include_prerelease {
                    // Find the set of versions that are allowed to have prereleases
                    // For example, ^1.2.3-pr.1 desugars to >=1.2.3-pr.1 <2.0.0
                    // That should allow `1.2.3-pr.2` to pass.
                    // However, `1.2.4-alpha.notready` should NOT be allowed,
                    // even though it's within the range set by the comparators.
                    for c in comparators.iter() {
                        let v = &c.version;
                        if v.is_any() {
                            continue;
                        }

                        if v.has_prerelease() && version.major == v.major
                                && version.minor == v.minor
                                && version.patch == v.patch {
                            return true;
                        }
                    }

                    false
                } else {
                    true
                }
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_hyphens() {
        let v = vec![("1.2.3 - 1.2.4", ">=1.2.3 <=1.2.4")];
        for v in v {
            let res = Range::replace_hyphens(v.0, false).unwrap();
            let comp = format!("{}", &res.unwrap());
            assert!(!comp.contains('-'), "contains hyphen");
            assert_eq!(comp, String::from(v.1));
        }
    }

    #[test]
    fn trim_operators() {
        let v = vec![("> 1.2.3 < 1.2.5", ">1.2.3 <1.2.5")];
        for v in v {
            let res = Range::trim_operators(v.0);
            assert_eq!(res, String::from(v.1));
        }
    }

    #[test]
    fn trim_tilde() {
        let v = vec![("~ 1.2.3", "~1.2.3")];
        for v in v {
            let res = Range::trim_tilde(v.0);
            assert_eq!(res, String::from(v.1));
        }
    }

    #[test]
    fn trim_caret() {
        let v = vec![("^ 1.2.3", "^1.2.3")];
        for v in v {
            let res = Range::trim_caret(v.0);
            assert_eq!(res, String::from(v.1));
        }
    }

    #[test]
    fn trim_spaces() {
        let v = vec![("1.2.3    1.2.4", "1.2.3 1.2.4")];
        for v in v {
            let res = Range::trim_spaces(v.0);
            assert_eq!(res, String::from(v.1));
        }
    }

    #[test]
    fn replce_carets() {
        let v = vec![("^1.2.3", ">=1.2.3 <2.0.0")];
        for v in v {
            let res = Range::replace_carets(v.0).unwrap().unwrap();
            assert_eq!(res.to_string(), String::from(v.1));
        }
    }
}
