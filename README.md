# SemVer

[![Build Status](https://travis-ci.org/gngeorgiev/semver_rs.svg?branch=master)](https://travis-ci.org/gngeorgiev/semver_rs)

Semantic version parsing and comparison ([semver](https://semver.org/)). The implementation of this crate is based on the
[node-semver](https://github.com/npm/node-semver#readme) npm package. The tests are taken directly
from node-semver's repo. This should make this crate equally good at parsing semver expressions as the
node package manager.

# Examples

#### Comparing two versions:
```rust
use semver_rs::Version;

// by constructing version instances manually
let ver1 = Version::new("2.0.0").parse()?;
let ver2 = Version::new("1.2.3").parse()?;

assert!(ver1 > ver2);

// by using the exported helper function
use semver_rs::compare;
use std::cmp::Ordering;

assert!(compare("2.0.0", "1.2.3", None)? == Ordering::Greater);
```

#### Checking whether a version is in a range
```rust
use semver_rs::{Range, Version};

// by constructing version instances manually
let range = Range::new(">=1.2.3").parse()?;
let ver = Version::new("1.2.4").parse()?;

assert!(range.test(&ver));

// by using the exported helper function
use semver_rs::satisfies;

assert!(satisfies("1.2.4", ">=1.2.4", None)?);
```

#### Parsing with specific options
```rust
use semver_rs::{Version, Range, Options};

let opts = Options::builder().loose(true).include_prerelease(true).build();
let range = Range::new(">=1.2.3").with_options(opts.clone()).parse()?;
let ver = Version::new("1.2.4-pre1").with_options(opts.clone()).parse()?;

assert!(range.test(&ver));
```

# Comparisons and considerations with other crates

At the time of writing this README there's only one other crate in the Rust ecosystem capable of parsing semver - [steveklabnik/semver](https://github.com/steveklabnik/semver).
While this crate is being used in cargo and is clearly doing its job there very well while comparing arbitrary semver
strings from a number of NPM packages I found it unable to parse a lot of them. Since its implementation of semver was vastly different
from NPM's I decided to base this crate on NPM's package in the hopes of making it easier to keep up with any updates in the future.
I kept the implementation as close as possible so the code structure and the way input is parsed should be very similar.
One day this crate can even become a drop-in replacement of node-semver by compiling it to WASM.

One tradeoff this implementation had to make was a tiny bit of performance. Since the parsing is based heavily on Regex it's a little bit slower - in the matter of few hundred nanoseconds.
There are still a lot of string allocations that can be eliminated, especially in parsing Ranges and Versions with prereleases.
I am not sure how many of these can be eliminated at the moment but any help would be greatly appreciated.
Performance metrics will be posted later on.
