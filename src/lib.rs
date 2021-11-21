#![doc(html_root_url = "https://docs.rs/semver-rs/0.1.0")]
#![deny(warnings, missing_debug_implementations)]

//! Semantic version parsing and comparison ([semver](https://semver.org/)). The implementation of this crate is based on the
//! [node-semver](https://github.com/npm/node-semver#readme) npm package. The tests are taken directly
//! from node-semver's repo. This should make this crate equally good at parsing semver expressions as the
//! node package manager.
//!
//! ## Examples
//!
//! #### Comparing two versions:
//! ```
//! use semver_rs::Version;
//!
//! // by constructing version instances manually
//! let ver1 = Version::new("2.0.0").parse()?;
//! let ver2 = Version::new("1.2.3").parse()?;
//!
//! assert!(ver1 > ver2);
//!
//! // by using the exported helper function
//! use semver_rs::compare;
//! use std::cmp::Ordering;
//!
//! assert!(compare("2.0.0", "1.2.3", None)? == Ordering::Greater);
//! # Ok::<(), semver_rs::Error>(())
//! ```
//!
//! #### Checking whether a version is in a range
//! ```
//! use semver_rs::{Range, Version};
//!
//! // by constructing version instances manually
//! let range = Range::new(">=1.2.3").parse()?;
//! let ver = Version::new("1.2.4").parse()?;
//!
//! assert!(range.test(&ver));
//!
//! // by using the exported helper function
//! use semver_rs::satisfies;
//!
//! assert!(satisfies("1.2.4", ">=1.2.4", None)?);
//! # Ok::<(), semver_rs::Error>(())
//! ```
//!
//! #### Parsing with specific options
//! ```
//! use semver_rs::{Version, Range, Options};
//!
//! let opts = Options::builder().loose(true).include_prerelease(true).build();
//! let range = Range::new(">=1.2.3").with_options(opts.clone()).parse()?;
//! let ver = Version::new("1.2.4-pre1").with_options(opts.clone()).parse()?;
//!
//! assert!(range.test(&ver));
//!
//! # Ok::<(), semver_rs::Error>(())
//! ```

mod builder;
mod comparator;
mod compare_fns;
mod error;
mod expressions;
mod operator;
mod range;
mod util;
mod version;

pub use builder::{Builder, Options, OptionsBuilder, Parseable};
pub use compare_fns::*;
pub use error::{Error, ErrorKind};
pub use operator::Operator;
pub use range::Range;
pub use version::Version;

#[cfg(test)]
mod tests {
    use crate::builder::Options;
    use crate::operator::Operator;
    use std::cmp::Ordering;

    #[test]
    fn clean() {
        let v = vec![
            ("1.2.3", "1.2.3"),
            (" 1.2.3  ", "1.2.3"),
            (" 1.2.3-4  ", "1.2.3-4"),
            (" 1.2.3-pre  ", "1.2.3-pre"),
            ("  =v1.2.3   ", "1.2.3"),
            ("v1.2.3", "1.2.3"),
            ("  v1.2.3 ", "1.2.3"),
            ("\t1.2.3", "1.2.3"),
            (">1.2.3", ""),
            ("~1.2.3", ""),
            ("<=1.2.3", ""),
            ("1.2.x", ""),
        ];

        for (input, output) in v {
            println!("testing: clean: {} {}", input, output);
            let res = super::clean(input, None).expect(input);
            assert_eq!(res, output, "{} => {}", input, output);
        }
    }

    #[test]
    fn compares() {
        //the first should be greater than the second
        let v = vec![
            ("0.0.0", "0.0.0-foo", false),
            ("0.0.1", "0.0.0", false),
            ("1.0.0", "0.9.9", false),
            ("0.10.0", "0.9.0", false),
            ("0.99.0", "0.10.0", true),
            ("2.0.0", "1.2.3", true),
            ("v0.0.0", "0.0.0-foo", true),
            ("v0.0.1", "0.0.0", true),
            ("v1.0.0", "0.9.9", true),
            ("v0.10.0", "0.9.0", true),
            ("v0.99.0", "0.10.0", true),
            ("v2.0.0", "1.2.3", true),
            ("0.0.0", "v0.0.0-foo", true),
            ("0.0.1", "v0.0.0", true),
            ("1.0.0", "v0.9.9", true),
            ("0.10.0", "v0.9.0", true),
            ("0.99.0", "v0.10.0", true),
            ("2.0.0", "v1.2.3", true),
            ("1.2.3", "1.2.3-asdf", false),
            ("1.2.3", "1.2.3-4", false),
            ("1.2.3", "1.2.3-4-foo", false),
            ("1.2.3-5-foo", "1.2.3-5", false),
            ("1.2.3-5", "1.2.3-4", false),
            ("1.2.3-5-foo", "1.2.3-5-Foo", false),
            ("3.0.0", "2.7.2+asdf", false),
            ("1.2.3-a.10", "1.2.3-a.5", false),
            ("1.2.3-a.b", "1.2.3-a.5", false),
            ("1.2.3-a.b", "1.2.3-a", false),
            ("1.2.3-a.b.c.10.d.5", "1.2.3-a.b.c.5.d.100", false),
            ("1.2.3-r2", "1.2.3-r100", false),
            ("1.2.3-r100", "1.2.3-R2", false),
        ];

        for (v1, v2, loose) in v {
            println!("testing compares: {} {} loose: {}", v1, v2, loose);
            let opts = Options::builder().loose(loose).build();
            let res = super::compare(v1, v2, Some(opts)).unwrap();
            assert!(res == Ordering::Greater);
            assert!(super::compare(v1, v1, Some(opts)).unwrap() == Ordering::Equal);
            assert!(super::compare(v2, v2, Some(opts)).unwrap() == Ordering::Equal);
        }
    }

    #[test]
    fn equality() {
        let v = vec![
            ("1.2.3", "v1.2.3", true),
            ("1.2.3", "=1.2.3", true),
            ("1.2.3", "v 1.2.3", true),
            ("1.2.3", "= 1.2.3", true),
            ("1.2.3", " v1.2.3", true),
            ("1.2.3", " =1.2.3", true),
            ("1.2.3", " v 1.2.3", true),
            ("1.2.3", " = 1.2.3", true),
            ("1.2.3-0", "v1.2.3-0", true),
            ("1.2.3-0", "=1.2.3-0", true),
            ("1.2.3-0", "v 1.2.3-0", true),
            ("1.2.3-0", "= 1.2.3-0", true),
            ("1.2.3-0", " v1.2.3-0", true),
            ("1.2.3-0", " =1.2.3-0", true),
            ("1.2.3-0", " v 1.2.3-0", true),
            ("1.2.3-0", " = 1.2.3-0", true),
            ("1.2.3-1", "v1.2.3-1", true),
            ("1.2.3-1", "=1.2.3-1", true),
            ("1.2.3-1", "v 1.2.3-1", true),
            ("1.2.3-1", "= 1.2.3-1", true),
            ("1.2.3-1", " v1.2.3-1", true),
            ("1.2.3-1", " =1.2.3-1", true),
            ("1.2.3-1", " v 1.2.3-1", true),
            ("1.2.3-1", " = 1.2.3-1", true),
            ("1.2.3-beta", "v1.2.3-beta", true),
            ("1.2.3-beta", "=1.2.3-beta", true),
            ("1.2.3-beta", "v 1.2.3-beta", true),
            ("1.2.3-beta", "= 1.2.3-beta", true),
            ("1.2.3-beta", " v1.2.3-beta", true),
            ("1.2.3-beta", " =1.2.3-beta", true),
            ("1.2.3-beta", " v 1.2.3-beta", true),
            ("1.2.3-beta", " = 1.2.3-beta", true),
            ("1.2.3-beta+build", " = 1.2.3-beta+otherbuild", true),
            ("1.2.3+build", " = 1.2.3+otherbuild", true),
            ("1.2.3-beta+build", "1.2.3-beta+otherbuild", false),
            ("1.2.3+build", "1.2.3+otherbuild", false),
            ("  v1.2.3+build", "1.2.3+otherbuild", false),
        ];

        for (v1, v2, loose) in v {
            println!("testing equality: {} {} loose: {}", v1, v2, loose);
            let opts = Options::builder().loose(loose).build();
            let res = super::compare(v1, v2, Some(opts)).unwrap();
            assert!(res == Ordering::Equal);
            assert!(super::cmp(v1, Operator::Gte, v2, Some(opts)).unwrap());
            assert!(super::cmp(v1, Operator::Lte, v2, Some(opts)).unwrap());
        }
    }

    #[test]
    fn satisfies() {
        let v = vec![
            ("1.0.0 - 2.0.0", "1.2.3", false),
            ("^1.2.3+build", "1.2.3", false),
            ("^1.2.3+build", "1.3.0", false),
            ("1.2.3-pre+asdf - 2.4.3-pre+asdf", "1.2.3", false),
            ("1.2.3pre+asdf - 2.4.3-pre+asdf", "1.2.3", true),
            ("1.2.3-pre+asdf - 2.4.3pre+asdf", "1.2.3", true),
            ("1.2.3pre+asdf - 2.4.3pre+asdf", "1.2.3", true),
            ("1.2.3-pre+asdf - 2.4.3-pre+asdf", "1.2.3-pre.2", false),
            ("1.2.3-pre+asdf - 2.4.3-pre+asdf", "2.4.3-alpha", false),
            ("1.2.3+asdf - 2.4.3+asdf", "1.2.3", false),
            ("1.0.0", "1.0.0", false),
            (">=*", "0.2.4", false),
            ("", "1.0.0", false),
            ("*", "1.2.3", true),
            ("*", "v1.2.3", true),
            (">=1.0.0", "1.0.0", true),
            (">=1.0.0", "1.0.1", true),
            (">=1.0.0", "1.1.0", true),
            (">1.0.0", "1.0.1", true),
            (">1.0.0", "1.1.0", false),
            ("<=2.0.0", "2.0.0", false),
            ("<=2.0.0", "1.9999.9999", false),
            ("<=2.0.0", "0.2.9", false),
            ("<2.0.0", "1.9999.9999", false),
            ("<2.0.0", "0.2.9", false),
            (">= 1.0.0", "1.0.0", false),
            (">=  1.0.0", "1.0.1", false),
            (">=   1.0.0", "1.1.0", false),
            ("> 1.0.0", "1.0.1", false),
            (">  1.0.0", "1.1.0", false),
            ("<=   2.0.0", "2.0.0", false),
            ("<= 2.0.0", "1.9999.9999", false),
            ("<=  2.0.0", "0.2.9", false),
            ("<    2.0.0", "1.9999.9999", false),
            ("<     2.0.0", "0.2.9", false),
            (">=0.1.97", "v0.1.97", true),
            (">=0.1.97", "0.1.97", false),
            ("0.1.20 || 1.2.4", "1.2.4", false),
            (">=0.2.3 || <0.0.1", "0.0.0", false),
            (">=0.2.3 || <0.0.1", "0.2.3", false),
            (">=0.2.3 || <0.0.1", "0.2.4", false),
            ("||", "1.3.4", false),
            ("2.x.x", "2.1.3", false),
            ("1.2.x", "1.2.3", false),
            ("1.2.x || 2.x", "2.1.3", false),
            ("1.2.x || 2.x", "1.2.3", false),
            ("x", "1.2.3", false),
            ("2.*.*", "2.1.3", false),
            ("1.2.*", "1.2.3", false),
            ("1.2.* || 2.*", "2.1.3", false),
            ("1.2.* || 2.*", "1.2.3", false),
            ("*", "1.2.3", false),
            ("2", "2.1.2", false),
            ("2.3", "2.3.1", false),
            ("~x", "0.0.9", false),
            ("~2", "2.0.9", false),
            ("~2.4", "2.4.0", false),
            ("~2.4", "2.4.5", false),
            ("~>3.2.1", "3.2.2", false),
            ("~1", "1.2.3", false),
            ("~>1", "1.2.3", false),
            ("~> 1", "1.2.3", false),
            ("~1.0", "1.0.2", false),
            ("~ 1.0", "1.0.2", false),
            ("~ 1.0.3", "1.0.12", false),
            (">=1", "1.0.0", false),
            (">= 1", "1.0.0", false),
            ("<1.2", "1.1.1", false),
            ("< 1.2", "1.1.1", false),
            ("~v0.5.4-pre", "0.5.5", false),
            ("~v0.5.4-pre", "0.5.4", false),
            ("=0.7.x", "0.7.2", false),
            ("<=0.7.x", "0.7.2", false),
            (">=0.7.x", "0.7.2", false),
            ("<=0.7.x", "0.6.2", false),
            ("~1.2.1 >=1.2.3", "1.2.3", false),
            ("~1.2.1 =1.2.3", "1.2.3", false),
            ("~1.2.1 1.2.3", "1.2.3", false),
            ("~1.2.1 >=1.2.3 1.2.3", "1.2.3", false),
            ("~1.2.1 1.2.3 >=1.2.3", "1.2.3", false),
            ("~1.2.1 1.2.3", "1.2.3", false),
            (">=1.2.1 1.2.3", "1.2.3", false),
            ("1.2.3 >=1.2.1", "1.2.3", false),
            (">=1.2.3 >=1.2.1", "1.2.3", false),
            (">=1.2.1 >=1.2.3", "1.2.3", false),
            (">=1.2", "1.2.8", false),
            ("^1.2.3", "1.8.1", false),
            ("^0.1.2", "0.1.2", false),
            ("^0.1", "0.1.2", false),
            ("^0.0.1", "0.0.1", false),
            ("^1.2", "1.4.2", false),
            ("^1.2 ^1", "1.4.2", false),
            ("^1.2.3-alpha", "1.2.3-pre", false),
            ("^1.2.0-alpha", "1.2.0-pre", false),
            ("^0.0.1-alpha", "0.0.1-beta", false),
            ("^0.1.1-alpha", "0.1.1-beta", false),
            ("^x", "1.2.3", false),
            ("x - 1.0.0", "0.9.7", false),
            ("x - 1.x", "0.9.7", false),
            ("1.0.0 - x", "1.9.7", false),
            ("1.x - x", "1.9.7", false),
            ("<=7.x", "7.9.9", false),
        ];

        for (range, ver, loose) in v {
            println!("testing satisfies: {} {} loose: {}", range, ver, loose);
            let opts = Options::builder().loose(loose).build();
            let res = super::satisfies(ver, range, Some(opts)).unwrap();
            assert!(res);
        }
    }

    #[test]
    fn negative_satisfies() {
        let v = vec![
            ("1.0.0 - 2.0.0", "2.2.3", false),
            ("1.2.3+asdf - 2.4.3+asdf", "1.2.3-pre.2", false),
            ("1.2.3+asdf - 2.4.3+asdf", "2.4.3-alpha", false),
            ("^1.2.3+build", "2.0.0", false),
            ("^1.2.3+build", "1.2.0", false),
            ("^1.2.3", "1.2.3-pre", false),
            ("^1.2", "1.2.0-pre", false),
            (">1.2", "1.3.0-beta", false),
            ("<=1.2.3", "1.2.3-beta", false),
            ("^1.2.3", "1.2.3-beta", false),
            ("=0.7.x", "0.7.0-asdf", false),
            (">=0.7.x", "0.7.0-asdf", false),
            ("1", "1.0.0beta", true),
            ("<1", "1.0.0beta", true),
            ("< 1", "1.0.0beta", true),
            ("1.0.0", "1.0.1", false),
            (">=1.0.0", "0.0.0", false),
            (">=1.0.0", "0.0.1", false),
            (">=1.0.0", "0.1.0", false),
            (">1.0.0", "0.0.1", false),
            (">1.0.0", "0.1.0", false),
            ("<=2.0.0", "3.0.0", false),
            ("<=2.0.0", "2.9999.9999", false),
            ("<=2.0.0", "2.2.9", false),
            ("<2.0.0", "2.9999.9999", false),
            ("<2.0.0", "2.2.9", false),
            (">=0.1.97", "v0.1.93", true),
            (">=0.1.97", "0.1.93", false),
            ("0.1.20 || 1.2.4", "1.2.3", false),
            (">=0.2.3 || <0.0.1", "0.0.3", false),
            (">=0.2.3 || <0.0.1", "0.2.2", false),
            ("2.x.x", "1.1.3", true),
            ("2.x.x", "3.1.3", false),
            ("1.2.x", "1.3.3", false),
            ("1.2.x || 2.x", "3.1.3", false),
            ("1.2.x || 2.x", "1.1.3", false),
            ("2.*.*", "1.1.3", false),
            ("2.*.*", "3.1.3", false),
            ("1.2.*", "1.3.3", false),
            ("1.2.* || 2.*", "3.1.3", false),
            ("1.2.* || 2.*", "1.1.3", false),
            ("2", "1.1.2", false),
            ("2.3", "2.4.1", false),
            ("~2.4", "2.5.0", false),
            ("~2.4", "2.3.9", false),
            ("~>3.2.1", "3.3.2", false),
            ("~>3.2.1", "3.2.0", false),
            ("~1", "0.2.3", false),
            ("~>1", "2.2.3", false),
            ("~1.0", "1.1.0", false),
            ("<1", "1.0.0", false),
            (">=1.2", "1.1.1", false),
            ("1", "2.0.0beta", true),
            ("~v0.5.4-beta", "0.5.4-alpha", false),
            ("=0.7.x", "0.8.2", false),
            (">=0.7.x", "0.6.2", false),
            ("<0.7.x", "0.7.2", false),
            ("<1.2.3", "1.2.3-beta", false),
            ("=1.2.3", "1.2.3-beta", false),
            (">1.2", "1.2.8", false),
            ("^0.0.1", "0.0.2", false),
            ("^1.2.3", "2.0.0-alpha", false),
            ("^1.2.3", "1.2.2", false),
            ("^1.2", "1.1.9", false),
            ("*", "v1.2.3-foo", true),
            ("blerg", "1.2.3", false),
            (
                "git+https://user:password0123@github.com/foo",
                "123.0.0",
                true,
            ),
            ("^1.2.3", "2.0.0-pre", false),
            ("^1.2.3", "false", false),
        ];

        for (range, ver, loose) in v {
            println!(
                "testing satisfies_negative: {} {} loose: {}",
                range, ver, loose
            );
            let opts = Options::builder().loose(loose).build();
            let res = super::satisfies(ver, range, Some(opts)).unwrap_or(false);
            assert!(!res);
        }
    }

    #[test]
    fn unlocked_prerelease_range() {
        let v = vec![
            ("*", "1.0.0-rc1"),
            ("^1.0.0", "2.0.0-rc1"),
            ("^1.0.0-0", "1.0.1-rc1"),
            ("^1.0.0-rc2", "1.0.1-rc1"),
            ("^1.0.0", "1.0.1-rc1"),
            ("^1.0.0", "1.1.0-rc1"),
        ];

        for (range, ver) in v {
            println!("testing unlocked_prerelease_range {} {}", &range, &ver);
            let opts = Options::builder().include_prerelease(true).build();
            let res = super::satisfies(ver, range, Some(opts)).unwrap();
            assert!(res);
        }
    }

    #[test]
    fn negative_unlocked_prerelease_range() {
        let v = vec![("^1.0.0", "1.0.0-rc1"), ("^1.2.3-rc2", "2.0.0")];

        for (range, ver) in v {
            println!("testing unlocked_prerelease_range {} {}", &range, &ver);
            let opts = Options::builder().include_prerelease(true).build();
            let res = super::satisfies(ver, range, Some(opts)).unwrap();
            assert!(!res);
        }
    }
}
