use super::{
    comparator::Comparator,
    expressions::{
        RANGE_HYPHEN, RANGE_OR, RANGE_TRIM_CARET, RANGE_TRIM_OPERATORS, RANGE_TRIM_TILDE,
        SPLIT_SPACES,
    },
    operator::Operator,
    util::{is_any_version, match_at_index},
};

use failure::Error;

pub struct Range {
    comparators: Vec<Comparator>,
}

impl Range {
    pub fn new<S: Into<String>>(range: S) -> Result<Range, Error> {
        let range = range.into();
        let comparators: Result<Vec<Comparator>, Error> = RANGE_OR
            .split(&range)
            .map(|range| {
                //1. trim the range
                let mut range = range.trim().to_owned();
                //2. replace hyphens `1.2.3 - 1.2.4` => `>=1.2.3 <=1.2.4`
                range = Range::replace_hyphens(range)?;
                //3. trim the spaces around operators `> 1.2.3 < 1.2.5` => `>1.2.3 <1.2.5`
                range = Range::trim_operators(range);
                //4. trim spaces around the tilde operator `~ 1.2.3` => `~1.2.3`
                range = Range::trim_tilde(range);
                //5. trim spaces around the caret operator `^ 1.2.3` => `^1.2.3`
                range = Range::trim_caret(range);
                //6. trim all the spaces that are left `1.2.3  1.2.4` => `1.2.3 1.2.4`
                range = Range::trim_spaces(range);

                Comparator::new(range)
            }).collect();

        Ok(Range {
            comparators: comparators?,
        })
    }

    fn trim_spaces(range: String) -> String {
        let split: Vec<String> = SPLIT_SPACES.split(&range).map(|v| v.to_owned()).collect();
        split.join(" ")
    }

    fn trim_caret(range: String) -> String {
        RANGE_TRIM_CARET.replace_all(&range, "$1^").into()
    }

    fn trim_tilde(range: String) -> String {
        RANGE_TRIM_TILDE.replace_all(&range, "$1~").into()
    }

    fn trim_operators(range: String) -> String {
        RANGE_TRIM_OPERATORS.replace_all(&range, "$1$2$3").into()
    }

    fn replace_hyphens(range: String) -> Result<String, Error> {
        let cap = RANGE_HYPHEN.captures_iter(&range).collect::<Vec<_>>();
        let cap = cap.first().unwrap();

        let mut from = match_at_index(cap, 1);
        let from_major = match_at_index(cap, 2);
        let from_minor = match_at_index(cap, 3);
        let from_patch = match_at_index(cap, 4);

        let mut to = match_at_index(cap, 7);
        let to_major = match_at_index(cap, 8);
        let to_minor = match_at_index(cap, 9);
        let to_patch = match_at_index(cap, 10);
        let to_prerelease = match_at_index(cap, 11);

        if is_any_version(&from_major) {
            from = String::new();
        } else if is_any_version(&from_minor) {
            from = format!("{}{}.0.0", Operator::Gte, from_major);
        } else if is_any_version(&from_patch) {
            from = format!("{}{}.{}.0", Operator::Gte, from_major, from_minor);
        } else {
            from = format!("{}{}", Operator::Gte, from);
        }

        if is_any_version(&to_major) {
            to = String::new();
        } else if is_any_version(&to_minor) {
            let to_major = to_major.parse::<usize>()?;
            to = format!("{}{}.0.0", Operator::Lt, to_major + 1);
        } else if is_any_version(&to_patch) {
            let to_minor = to_minor.parse::<usize>()?;
            to = format!("{}{}.0", Operator::Lt, to_minor + 1);
        } else if to_prerelease != "" {
            to = format!(
                "{}{}.{}.{}-{}",
                Operator::Lte,
                to_major,
                to_minor,
                to_patch,
                to_prerelease
            );
        } else {
            to = format!("{}{}", Operator::Lte, to);
        }

        Ok(format!("{} {}", from, to))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_hyphens() {
        let v = vec![("1.2.3 - 1.2.4", ">=1.2.3 <=1.2.4")];
        for v in v {
            let res = Range::replace_hyphens(String::from(v.0)).unwrap();
            assert!(!res.contains("-"), "contains hyphen");
            assert_eq!(res, String::from(v.1));
        }
    }

    #[test]
    fn trim_operators() {
        let v = vec![("> 1.2.3 < 1.2.5", ">1.2.3 <1.2.5")];
        for v in v {
            let res = Range::trim_operators(String::from(v.0));
            assert_eq!(res, String::from(v.1));
        }
    }

    #[test]
    fn trim_tilde() {
        let v = vec![("~ 1.2.3", "~1.2.3")];
        for v in v {
            let res = Range::trim_tilde(String::from(v.0));
            assert_eq!(res, String::from(v.1));
        }
    }

    #[test]
    fn trim_caret() {
        let v = vec![("^ 1.2.3", "^1.2.3")];
        for v in v {
            let res = Range::trim_caret(String::from(v.0));
            assert_eq!(res, String::from(v.1));
        }
    }

    #[test]
    fn trim_spaces() {
        let v = vec![("1.2.3    1.2.4", "1.2.3 1.2.4")];
        for v in v {
            let res = Range::trim_spaces(String::from(v.0));
            assert_eq!(res, String::from(v.1));
        }
    }
}
