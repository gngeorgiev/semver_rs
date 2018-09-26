pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub prerelease: Vec<i32>,
    pub build: Vec<String>,
}
