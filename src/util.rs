use regex::{Captures, Match};

pub(crate) fn is_any_version(v: &str) -> bool {
    if v == "" || v == "*" || v.to_lowercase() == "x" {
        true
    } else {
        false
    }
}

pub(crate) fn match_at_index(v: &Captures, i: usize) -> String {
    v.get(i)
        .as_ref()
        .map_or(String::new(), |v| v.as_str().to_owned())
}
