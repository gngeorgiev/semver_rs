use semver_rs::{Options, Range, Version};

#[test]
fn test_compare() {
    let opts = Options::builder()
        .loose(true)
        .include_prerelease(true)
        .build();

    let range = Range::new(">=1.2.3").with_options(opts).parse().unwrap();
    let ver = Version::new("1.2.4-pre1")
        .with_options(opts)
        .parse()
        .unwrap();

    assert!(range.test(&ver));
}
