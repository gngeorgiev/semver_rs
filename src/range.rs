use crate::builder::{Builder, Options, Parseable};
use crate::comparator::Comparator;
use crate::error::{Error, ErrorKind};
use crate::expressions::{
    COMPARATOR_LOOSE, COMP_REPLACE_CARETS, RANGE_HYPHEN, RANGE_HYPHEN_LOOSE, RANGE_OR,
    RANGE_TRIM_CARET, RANGE_TRIM_OPERATORS, RANGE_TRIM_TILDE, SPLIT_SPACES,
};
use crate::operator::Operator;
use crate::util::{is_any_version, match_at_index};
use crate::version::Version;

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
///
/// Currently the `Comparator` interface is not publicly available but might be exported in the future.
#[derive(Debug)]
pub struct Range {
    pub(crate) comparators: Vec<Vec<Comparator>>,

    opts: Option<Options>,
}

impl<'p> Parseable<'p> for Range {
    fn parse(range_input: &'p str, opts: Option<Options>) -> Result<Self, Error> {
        let loose = opts.clone().unwrap_or_default().loose;

        if range_input.len() == 0 {
            let comp = Comparator::empty();
            return Ok(Range {
                comparators: vec![vec![comp]],
                opts,
            });
        }

        let comparators_opts = opts.clone();
        let comparators_result: Result<Vec<Option<Vec<Comparator>>>, Error> = RANGE_OR
            .split(&range_input)
            .map(move |range| {
                //1. trim the range
                let mut range = range.trim().to_owned();
                //2. replace hyphens `1.2.3 - 1.2.4` => `>=1.2.3 <=1.2.4`
                range = Range::replace_hyphens(&range, loose)?;
                //3. trim the spaces around operators `> 1.2.3 < 1.2.5` => `>1.2.3 <1.2.5`
                range = Range::trim_operators(&range);
                //4. trim spaces around the tilde operator `~ 1.2.3` => `~1.2.3`
                range = Range::trim_tilde(&range);
                //5. trim spaces around the caret operator `^ 1.2.3` => `^1.2.3`
                range = Range::trim_caret(&range);
                //6. trim all the spaces that are left `1.2.3  1.2.4` => `1.2.3 1.2.4`
                range = Range::trim_spaces(&range);
                //7. replace the carets and adjust versions `^1.2.3 ---> >=1.2.3 <2.0.0`
                range = Range::replace_carets(&range)?;

                let comparators_parsed: Vec<String> = range
                    .split(" ")
                    .map(|c| Comparator::normalize(c, loose))
                    .collect::<Vec<_>>();

                let comparators_parsed = comparators_parsed.join(" ");
                if comparators_parsed.len() == 0 {
                    let comp = Comparator::empty();
                    return Ok(Some(vec![comp]));
                }

                // TODO: this split should yield an array with one empty string inside
                // when used on an empty string, just like in the original npm package
                // the condition above is a workaround atm
                let comparators_parsed: Vec<&str> =
                    SPLIT_SPACES.split(&comparators_parsed).collect();

                let opts = comparators_opts.clone();
                let comparators = comparators_parsed
                    .into_iter()
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
                        if comp.len() > 0 {
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
            comparators_result?.into_iter().filter_map(|c| c).collect();

        if comparators.len() == 0 {
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

    fn trim_spaces(range: &str) -> String {
        let split: Vec<String> = SPLIT_SPACES.split(range).map(|v| v.to_owned()).collect();
        split.join(" ")
    }

    fn trim_caret(range: &str) -> String {
        RANGE_TRIM_CARET.replace_all(range, "$1^").into()
    }

    fn trim_tilde(range: &str) -> String {
        RANGE_TRIM_TILDE.replace_all(range, "$1~").into()
    }

    fn trim_operators(range: &str) -> String {
        RANGE_TRIM_OPERATORS.replace_all(range, "$1$2$3").into()
    }

    fn replace_hyphens(range: &str, loose: bool) -> Result<String, Error> {
        let cap: Vec<_> = match loose {
            true => RANGE_HYPHEN_LOOSE.captures_iter(range).collect(),
            false => RANGE_HYPHEN.captures_iter(range).collect(),
        };
        let cap = match cap.first() {
            Some(cap) => cap,
            None => return Ok(range.to_owned()),
        };

        let mut from = match_at_index(cap, 1);
        let from_major = match_at_index(cap, 2);
        let from_minor = match_at_index(cap, 3);
        let from_patch = match_at_index(cap, 4);

        let mut to = match_at_index(cap, 7);
        let to_major = match_at_index(cap, 8);
        let to_minor = match_at_index(cap, 9);
        let to_patch = match_at_index(cap, 10);
        let to_prerelease = match_at_index(cap, 11);

        if is_any_version(&from_major) {
            from = String::new();
        } else if is_any_version(&from_minor) {
            from = format!("{}{}.0.0", Operator::Gte, from_major);
        } else if is_any_version(&from_patch) {
            from = format!("{}{}.{}.0", Operator::Gte, from_major, from_minor);
        } else {
            from = format!("{}{}", Operator::Gte, from);
        }

        if is_any_version(&to_major) {
            to = String::new();
        } else if is_any_version(&to_minor) {
            let to_major = to_major.parse::<usize>()?;
            to = format!("{}{}.0.0", Operator::Lt, to_major + 1);
        } else if is_any_version(&to_patch) {
            let to_minor = to_minor.parse::<usize>()?;
            to = format!("{}{}.0", Operator::Lt, to_minor + 1);
        } else if to_prerelease != "" {
            to = format!(
                "{}{}.{}.{}-{}",
                Operator::Lte,
                to_major,
                to_minor,
                to_patch,
                to_prerelease
            );
        } else {
            to = format!("{}{}", Operator::Lte, to);
        }

        Ok(format!("{} {}", from, to))
    }

    fn replace_carets(range: &str) -> Result<String, Error> {
        let cap = COMP_REPLACE_CARETS.captures_iter(range).collect::<Vec<_>>();
        let cap = match cap.first() {
            Some(cap) => cap,
            None => return Ok(range.to_owned()),
        };

        let major = match_at_index(cap, 1);
        let minor = match_at_index(cap, 2);
        let patch = match_at_index(cap, 3);
        let mut prerelease = match_at_index(cap, 4);

        if is_any_version(&major) {
            Ok(String::new())
        } else if is_any_version(&minor) {
            let major: usize = major.parse()?;
            Ok(format!(
                "{}{}.0.0 {}{}.0.0",
                Operator::Gte,
                major,
                Operator::Lt,
                major + 1,
            ))
        } else if is_any_version(&patch) {
            let minor: usize = minor.parse()?;
            if major == "0" {
                Ok(format!(
                    "{}{}.{}.0 {}{}.{}.0",
                    Operator::Gte,
                    major,
                    minor,
                    Operator::Lt,
                    major,
                    minor + 1,
                ))
            } else {
                let major: usize = major.parse()?;
                Ok(format!(
                    "{}{}.{}.0 {}{}.0.0",
                    Operator::Gte,
                    major,
                    minor,
                    Operator::Lt,
                    major + 1,
                ))
            }
        } else if prerelease != "" {
            //this unwrap will never panic since we already verified that we have at least
            //one char in the string
            prerelease = if prerelease.chars().next().unwrap() == '-' {
                prerelease
            } else {
                format!("-{}", prerelease)
            };

            if major == "0" {
                if minor == "0" {
                    let patch: usize = patch.parse()?;
                    Ok(format!(
                        "{}{}.{}.{}{} {}{}.{}.{}",
                        Operator::Gte,
                        major,
                        minor,
                        patch,
                        prerelease,
                        Operator::Lt,
                        major,
                        minor,
                        patch + 1
                    ))
                } else {
                    let minor: usize = minor.parse()?;
                    Ok(format!(
                        "{}{}.{}.{}{} {}{}.{}.0",
                        Operator::Gte,
                        major,
                        minor,
                        patch,
                        prerelease,
                        Operator::Lt,
                        major,
                        minor + 1,
                    ))
                }
            } else {
                let major: usize = major.parse()?;
                Ok(format!(
                    "{}{}.{}.{}{} {}{}.0.0",
                    Operator::Gte,
                    major,
                    minor,
                    patch,
                    prerelease,
                    Operator::Lt,
                    major + 1,
                ))
            }
        } else {
            if major == "0" {
                if minor == "0" {
                    let patch: usize = patch.parse()?;
                    Ok(format!(
                        "{}{}.{}.{} {}{}.{}.{}",
                        Operator::Gte,
                        major,
                        minor,
                        patch,
                        Operator::Lt,
                        major,
                        minor,
                        patch + 1
                    ))
                } else {
                    let minor: usize = minor.parse()?;
                    Ok(format!(
                        "{}{}.{}.{} {}{}.{}.0",
                        Operator::Gte,
                        major,
                        minor,
                        patch,
                        Operator::Lt,
                        major,
                        minor + 1,
                    ))
                }
            } else {
                let major: usize = major.parse()?;
                Ok(format!(
                    "{}{}.{}.{} {}{}.0.0",
                    Operator::Gte,
                    major,
                    minor,
                    patch,
                    Operator::Lt,
                    major + 1,
                ))
            }
        }
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
                    if !c.test(&version) {
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

                        if v.has_prerelease() {
                            if v.major == v.major && v.minor == v.minor && v.patch == v.patch {
                                return true;
                            }
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
            assert!(!res.contains("-"), "contains hyphen");
            assert_eq!(res, String::from(v.1));
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
            let res = Range::replace_carets(v.0).unwrap();
            assert_eq!(res, String::from(v.1));
        }
    }
}
