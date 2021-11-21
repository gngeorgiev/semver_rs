# SemVer

[![semver_rs](https://docs.rs/semver_rs/badge.svg)](https://docs.rs/semver_rs) [![semver_rs](https://img.shields.io/crates/v/semver_rs.svg)](https://crates.io/crates/semver_rs) [![Build Status](https://travis-ci.org/gngeorgiev/semver_rs.svg?branch=master)](https://travis-ci.org/gngeorgiev/semver_rs)

Semantic version parsing and comparison ([semver](https://semver.org/)). The implementation of this crate is based on the
[node-semver](https://github.com/npm/node-semver#readme) npm package. The tests are taken directly
from node-semver's repo. This should make this crate as good at parsing semver expressions as the
node package manager.

## Installation

Add this to your `[dependencies]` section in `Cargo.toml`:

```toml
semver_rs = "0.1"
```

## Usage

### Comparing two versions

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

### Checking whether a version is in a range

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

### Parsing with specific options

```rust
use semver_rs::{Version, Range, Options};

let opts = Options::builder().loose(true).include_prerelease(true).build();
let range = Range::new(">=1.2.3").with_options(opts).parse()?;
let ver = Version::new("1.2.4-pre1").with_options(opts).parse()?;

assert!(range.test(&ver));
```

### Serializing

In order to allow serializing the semver structs allow the `serde` feature:

```toml
semver_rs = { version = "0.1", features = ["serde"] }
```

```rust
use semver_rs::{Range, Options};

let opts = Options::builder().loose(true).include_prerelease(true).build();
let range = Range::new(">=1.2.3").with_options(opts).parse().unwrap();
let _ = serde_json::to_string(&opts).unwrap();
```

## Development

Install [just](https://github.com/casey/just) and run the setup:

```shell
cargo install just && just setup
```

## Run bench

To run the benchmarks populating the next point run:

```shell
pushd bench
bash gen.sh
cat combined.txt
popd
```

This shell script collects some ranges from random npm packages and compares the results for the three implementations -
`semver_node`, `semver_rs` and `steveklabnik/semver`. From the table bellow the results can be observed.

## Comparisons and considerations with other crates

At the time of writing this README there's only one other crate in the Rust ecosystem capable of parsing semver - [steveklabnik/semver](https://github.com/steveklabnik/semver).
While this crate is being used in cargo and is clearly doing its job there very well, while comparing arbitrary semver
strings from a number of NPM packages I found it unable to parse a lot of them. Since its implementation of semver was vastly different
from NPM's I decided to base this crate on NPM's package in the hopes of making it easier to keep up with any updates in the future.
I kept the implementation as close as possible so the code structure and the way input is parsed should be very similar.

One trade-off this implementation had to make was a tiny bit of performance. Since the parsing is based heavily on Regex it's a little bit slower.
There are still a lot of string allocations that can be eliminated, especially in parsing Ranges and Versions with prereleases.

```shell
┌─────────┬───────────────────────┬───────────┬───────────────┬────────┬────────────────────┐
│ (index) │         name          │ satisfies │ not_satisfies │ errors │     average_us     │
├─────────┼───────────────────────┼───────────┼───────────────┼────────┼────────────────────┤
│    0    │     'semver_node'     │    14     │      450      │   1    │ 36.763440860215056 │
│    1    │      'semver_rs'      │    14     │      450      │   1    │  9.5247311827957   │
│    2    │ 'steveklabnik/semver' │    11     │      444      │   10   │ 0.3311827956989247 │
└─────────┴───────────────────────┴───────────┴───────────────┴────────┴────────────────────┘
```

In conclussion `semver_rs` is faster than `semver_node` and slower than `steveklabnik/semver`. It's also as accurate
in parsing as `semver_node`, while `steveklabnik/semver` couldn't handle 9 of the ranges.

## Goals and the future

This initial release aims at bringing this package out in the public and also as a baseline in terms of performance.
From now on performance can be improved upon by keeping compatibility at the absolute maximum.
In general the parsing algorithm itself is not optimal at a lot of places. There's also a lot of needless string and vector
allocations at the moment which are leftovers from the prototyping phase of the package, that can be addressed gradually.
