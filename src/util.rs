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
