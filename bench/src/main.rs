use alloc_counter::count_alloc;
use alloc_counter::AllocCounterSystem;

#[global_allocator]
static A: AllocCounterSystem = AllocCounterSystem;

use easybench::bench;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

use semver::{Version as VersionOld, VersionReq};
use semver_rs::{Range, Version};

fn main() {
    let app = clap::App::new("bench")
        .arg(clap::Arg::with_name("semver_rs").long("semver_rs"))
        .arg(clap::Arg::with_name("semver").long("semver"))
        .arg(clap::Arg::with_name("alloc").long("alloc"))
        .get_matches();

    let file = File::open("./ranges.txt").unwrap();

    if app.is_present("semver_rs") {
        let _ = Range::new(">=1.2.3").parse(); //warmup
        for line in BufReader::new(file).lines() {
            let range = line.unwrap();
            let now = SystemTime::now();

            let ver = Version::new("3.0.0").parse().unwrap();
            let (satisfies, exception) = match Range::new(&range).parse() {
                Ok(r) => (r.test(&ver), false),
                Err(_) => (false, true),
            };

            println!(
                "{},{},{},{}",
                range,
                satisfies,
                exception,
                now.elapsed().unwrap().as_micros()
            );
        }
    } else if app.is_present("semver") {
        let _ = VersionReq::parse(">=1.2.3"); //warmup
        for line in BufReader::new(file).lines() {
            let range = line.unwrap();
            let now = SystemTime::now();

            let ver = VersionOld::parse("3.0.0").unwrap();
            let (satisfies, exception) = match VersionReq::parse(&range) {
                Ok(r) => (r.matches(&ver), false),
                Err(_) => (false, true),
            };

            println!(
                "{},{},{},{}",
                range,
                satisfies,
                exception,
                now.elapsed().unwrap().as_micros()
            );
        }
    } else if app.is_present("alloc") {
        let _ = Version::new("1.2.3").parse().ok();
        let _ = Range::new(">=1.2.3").parse().ok();

        println!(
            "semver_rs::Version {:?}",
            count_alloc(|| Version::new("2").parse().ok()).0
        );
        println!(
            "semver_rs::Range {:?}",
            count_alloc(|| Range::new(">=1.2.3").parse().ok()).0
        );

        println!(
            "semver::Version {:?}",
            count_alloc(|| VersionOld::parse("1.2.3").ok()).0
        );

        println!(
            "semver::VersionReq {:?}",
            count_alloc(|| VersionReq::parse(">=1.2.3").ok()).0
        );

        println!(
            "semver_rs version: 1.2.3 {}",
            bench(|| Version::new("1.2.3").parse().ok())
        );
        println!(
            "semver_rs range: >=1.2.3 {}",
            bench(|| Range::new(">=1.2.3").parse().ok())
        );

        // let ver = Version::new("1.2.3").parse().ok();
        // let range = Range::new(">=1.2.3").parse().ok();
        // println!(
        //     "semver_rs satisfies: >=1.2.3 1.2.3 {}",
        //     bench(|| range.test(&ver))
        // );

        println!(
            "semver version: 1.2.3 {}",
            bench(|| VersionOld::parse("1.2.3").unwrap())
        );
        println!(
            "semver range: >=1.2.3 {}",
            bench(|| VersionReq::parse(">=1.2.3").unwrap())
        );

        let ver = VersionOld::parse("1.2.3").unwrap();
        let range = VersionReq::parse(">=1.2.3").unwrap();
        println!(
            "semver satisfies: >=1.2.3 1.2.3 {}",
            bench(|| range.matches(&ver))
        );
    }
}
