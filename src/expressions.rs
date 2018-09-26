use regex::Regex;

lazy_static! {
    pub static ref RANGE_OR: Regex = Regex::new(r"/\s*\|\|\s*/").unwrap();
}

lazy_static! {
    pub static ref RANGE_HYPHEN: Regex = Regex::new(r"^\s*([v=\s]*(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?)\s+-\s+([v=\s]*(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?)\s*$").unwrap();
}

lazy_static!{
    pub static ref RANGE_TRIM_OPERATORS: Regex = Regex::new(r"(\s*)((?:<|>)?=?)\s*([v=\s]*([0-9]+)\.([0-9]+)\.([0-9]+)(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?|[v=\s]*(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?)").unwrap();
}

lazy_static! {
    pub static ref RANGE_TRIM_TILDE: Regex = Regex::new(r"(\s*)(?:~>?)\s+").unwrap();
}

lazy_static! {
    pub static ref RANGE_TRIM_CARET: Regex = Regex::new(r"(\s*)(?:\^)\s+").unwrap();
}

lazy_static! {
    pub static ref SPLIT_SPACES: Regex = Regex::new(r"\s+").unwrap();
}
