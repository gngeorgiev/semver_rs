use semver_rs::{Options, Range, Version};

#[test]
fn test_serde() {
    let opts = Options::builder()
        .loose(true)
        .include_prerelease(true)
        .build();

    let range = Range::new(">=1.2.3").with_options(opts).parse().unwrap();
    let ver = Version::new("1.2.4-pre1")
        .with_options(opts)
        .parse()
        .unwrap();

    let _ = serde_json::to_string(&opts).unwrap();
    let _ = serde_json::to_string(&range).unwrap();
    let _ = serde_json::to_string(&ver).unwrap();
}
