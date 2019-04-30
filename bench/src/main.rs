//use alloc_counter::count_alloc;
//use alloc_counter::AllocCounterSystem;
//
//#[global_allocator]
//static A: AllocCounterSystem = AllocCounterSystem;

//use easybench::bench;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

fn main() {
    let app = clap::App::new("bench")
        .arg(clap::Arg::with_name("semver_rs").long("semver_rs"))
        .arg(clap::Arg::with_name("semver").long("semver"))
        .get_matches();

    let file = File::open("./ranges.txt").unwrap();

    if app.is_present("semver_rs") {
        use semver_rs::{Range, Version};

        Range::new(">=1.2.3").parse(); //warmup
        let ver = Version::new("3.0.0").parse().unwrap();
        for line in BufReader::new(file).lines().collect::<Vec<_>>() {
            let range = line.unwrap();
            let now = SystemTime::now();

            let (satisfies, exception) = match Range::new(&range).parse() {
                Ok(r) => {
                    (r.test(&ver), false)
                },
                Err(err) => {
                    (false, true)
                }
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
        use semver::{Version as VersionOld, VersionReq};

        VersionReq::parse(">=1.2.3"); //warmup
        let ver = VersionOld::parse("3.0.0").unwrap();
        for line in BufReader::new(file).lines() {
            let range = line.unwrap();
            let now = SystemTime::now();

            let (satisfies, exception) = match VersionReq::parse(&range) {
                Ok(r) => {
                    (r.matches(&ver), false)
                }
                Err(err) => {
                    (false, true)
                }
            };

            println!("{},{},{},{}", range, satisfies, exception, now.elapsed().unwrap().as_micros());
        }
    }


    // println!("{:?}", count_alloc(|| rr.captures_iter("pesho").next()).0);
    //    println!("{}", bench(|| rb.captures("pesho".as_bytes())));

    //    Version::new("1.2.3").parse().unwrap();
    //    Range::new(">=1.2.3").parse().unwrap();
    //
    //    println!("{:?}", count_alloc(|| Version::new("2").parse().unwrap()).0);
    //    println!(
    //        "{:?}",
    //        count_alloc(|| Range::new(">=1.2.3").parse().unwrap()).0
    //    );

    // println!(
    //     "{:?}",
    //     count_alloc(|| VersionOld::parse("1.2.3").unwrap()).0
    // );

    //    println!(
    //        "semver_rs version: 1.2.3 {}",
    //        bench(|| Version::new("1.2.3").parse().unwrap())
    //    );
    //    println!(
    //        "semver_rs range: >=1.2.3 {}",
    //        bench(|| Range::new(">=1.2.3").parse().unwrap())
    //    );
    //    let ver = Version::new("1.2.3").parse().unwrap();
    //    let range = Range::new(">=1.2.3").parse().unwrap();
    //    println!(
    //        "semver_rs satisfies: >=1.2.3 1.2.3 {}",
    //        bench(|| range.test(&ver, None))
    //    );

    //    println!(
    //        "semver version: 1.2.3 {}",
    //        bench(|| VersionOld::parse("1.2.3").unwrap())
    //    );
    //    println!(
    //        "semver range: >=1.2.3 {}",
    //        bench(|| VersionReq::parse(">=1.2.3").unwrap())
    //    );
    //    let ver = VersionOld::parse("1.2.3").unwrap();
    //    let range = VersionReq::parse(">=1.2.3").unwrap();
    //    println!(
    //        "semver satisfies: >=1.2.3 1.2.3 {}",
    //        bench(|| range.matches(&ver))
    //    );
}
