use std::{borrow::Cow, cmp::Ordering};

use regex::Captures;

pub(crate) fn is_any_version(v: &str) -> bool {
    v.is_empty() || v == "*" || unicase::eq(v, "x")
}

pub(crate) fn match_at_index<'a>(v: &'a Captures, i: usize) -> &'a str {
    v.get(i).map_or("", |v| v.as_str())
}

pub(crate) fn match_at_index_owned(v: &Captures, i: usize) -> String {
    v.get(i).map_or(String::new(), |v| v.as_str().to_owned())
}

pub(crate) fn increment_version(v: &str) -> usize {
    v.parse::<usize>().unwrap() + 1
}

pub(crate) fn get_prerelease_prefix(prerelease: &str) -> &'static str {
    if !prerelease.starts_with('-') {
        "-"
    } else {
        ""
    }
}

pub(crate) fn replacer<'a>(
    func: impl Fn(&[String]) -> Cow<'a, str>,
) -> impl Fn(&regex::Captures) -> Cow<'a, str> {
    move |cap: &regex::Captures| {
        let args: [String; 6] = [
            match_at_index_owned(cap, 0),
            match_at_index_owned(cap, 1),
            match_at_index_owned(cap, 2),
            match_at_index_owned(cap, 3),
            match_at_index_owned(cap, 4),
            match_at_index_owned(cap, 5),
        ];

        func(&args)
    }
}

pub(crate) fn compare_identifiers<S: AsRef<str>>(a: S, b: S) -> Ordering {
    let a = a.as_ref();
    let b = b.as_ref();

    match (a.parse::<i32>(), b.parse::<i32>()) {
        (Ok(_), Err(_)) => Ordering::Less,
        (Err(_), Ok(_)) => Ordering::Greater,
        (Err(_), Err(_)) => a.cmp(b),
        (Ok(a), Ok(b)) => a.cmp(&b),
    }
}
