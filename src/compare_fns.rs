use crate::builder::IntoOptionsMaybe;
use crate::error::Error;
use crate::expressions::CLEAN_VERSION;
use crate::operator::Operator;
use crate::range::Range;
use crate::version::Version;

use std::cmp::Ordering;

/// Parses a string into a [Version](crate::Version).
pub fn parse(version: &str, opts: impl IntoOptionsMaybe) -> Result<Version, Error> {
    Version::new(version).with_options(opts).parse()
}

/// Cleanups a semver string making it semver complaint.
pub fn clean(version: &str, opts: impl IntoOptionsMaybe) -> Result<String, Error> {
    let clean_version = CLEAN_VERSION.replace_all(version.trim(), "");

    Ok(parse(&clean_version, opts)?.to_string())
}

/// Compares the ordering of [Version](crate::Version) `a` vs [Version](crate::Version) `b`.
pub fn compare(a: &str, b: &str, opts: impl IntoOptionsMaybe) -> Result<Ordering, Error> {
    let a = parse(a, opts)?;
    let b = parse(b, opts)?;
    Ok(a.partial_cmp(&b).unwrap())
}

/// Compares whether [Version](crate::Version) `a` matches the semver operator against [Version](crate::Version) `b`.
pub fn cmp(a: &str, op: Operator, b: &str, opts: impl IntoOptionsMaybe) -> Result<bool, Error> {
    let r = match op {
        Operator::Eq | Operator::StrictEq | Operator::Empty => {
            compare(a, b, opts)? == Ordering::Equal
        }
        Operator::Ne | Operator::StrictNe => compare(a, b, opts)? != Ordering::Equal,
        Operator::Gt => compare(a, b, opts)? == Ordering::Greater,
        Operator::Gte => {
            let ord = compare(a, b, opts)?;
            ord == Ordering::Greater || ord == Ordering::Equal
        }
        Operator::Lt => compare(a, b, opts)? == Ordering::Less,
        Operator::Lte => {
            let ord = compare(a, b, opts)?;
            ord == Ordering::Less || ord == Ordering::Equal
        }
    };

    Ok(r)
}

/// Checks whether [Version](crate::Version) is in a [Range](crate::Range).
pub fn satisfies(ver: &str, range: &str, opts: impl IntoOptionsMaybe) -> Result<bool, Error> {
    let range = Range::new(range).with_options(opts).parse()?;
    let ver = Version::new(ver).with_options(opts).parse()?;
    Ok(range.test(&ver))
}
