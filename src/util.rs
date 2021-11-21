use std::cmp::Ordering;

use regex::Captures;

pub(crate) fn is_any_version(v: &str) -> bool {
    v.is_empty() || v == "*" || unicase::eq(v, "x")
}

pub(crate) fn match_at_index_str<'a>(v: &'a Captures, i: usize) -> &'a str {
    v.get(i).map_or("", |v| v.as_str())
}

pub(crate) fn match_at_index(v: &Captures, i: usize) -> String {
    v.get(i).map_or(String::new(), |v| v.as_str().to_owned())
}

pub(crate) fn increment_version(v: &str) -> String {
    let v = v.parse::<usize>().unwrap();
    format!("{}", v + 1)
}

pub(crate) fn ensure_prerelease_dash(prerelease: &str) -> String {
    if !prerelease.starts_with('-') {
        format!("-{}", prerelease)
    } else {
        String::from(prerelease)
    }
}

pub(crate) fn replacer<'a>(
    func: impl Fn(Vec<String>) -> String + 'a,
) -> impl Fn(&regex::Captures) -> String + 'a {
    move |cap: &regex::Captures| {
        let mut args: Vec<String> = vec![];
        for i in 0..6 {
            args.push(match_at_index(cap, i))
        }

        func(args)
    }
}

pub(crate) fn compare_identifiers<S: Into<String>>(a: S, b: S) -> Ordering {
    let a = a.into();
    let b = b.into();

    let a_num = a.parse::<i32>();
    let b_num = b.parse::<i32>();

    if a_num.is_ok() && b_num.is_err() {
        Ordering::Less
    } else if b_num.is_ok() && a_num.is_err() {
        Ordering::Greater
    } else if a_num.is_err() && b_num.is_err() {
        a.cmp(&b)
    } else {
        a_num.unwrap().cmp(&b_num.unwrap())
    }
}
