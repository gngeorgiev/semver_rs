use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref SPLIT_SPACES: Regex = Regex::new(r"\s+").unwrap();

    pub static ref RANGE_OR: Regex = Regex::new(r"\s*\|\|\s*").unwrap();

    pub static ref RANGE_HYPHEN: Regex = Regex::new(r"^\s*([v=\s]*(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?)\s+-\s+([v=\s]*(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?)\s*$").unwrap();
    pub static ref RANGE_HYPHEN_LOOSE: Regex = Regex::new(r"^\s*([v=\s]*([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?)\s+-\s+([v=\s]*([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?)\s*$").unwrap();

    pub static ref RANGE_TRIM_OPERATORS: Regex = Regex::new(r"(\s*)((?:<|>)?=?)\s*([v=\s]*([0-9]+)\.([0-9]+)\.([0-9]+)(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?|[v=\s]*(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?)").unwrap();
    pub static ref RANGE_TRIM_TILDE: Regex = Regex::new(r"(\s*)(?:~>?)\s+").unwrap();
    pub static ref RANGE_TRIM_CARET: Regex = Regex::new(r"(\s*)(?:\^)\s+").unwrap();

    pub static ref COMP_REPLACE_CARETS: Regex = Regex::new(r"^(?:\^)[v=\s]*(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?$").unwrap();
    pub static ref COMP_REPLACE_CARETS_LOOSE: Regex = Regex::new(r"^(?:\^)[v=\s]*([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?$").unwrap();

    pub static ref COMP_REPLACE_TILDES: Regex = Regex::new(r"^(?:~>?)[v=\s]*(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?$").unwrap();
    pub static ref COMP_REPLACE_TILDES_LOOSE: Regex = Regex::new(r"^(?:~>?)[v=\s]*([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?$").unwrap();

    pub static ref COMP_REPLACE_XRANGES: Regex = Regex::new(r"^((?:<|>)?=?)\s*[v=\s]*(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:\.(0|[1-9]\d*|x|X|\*)(?:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?$").unwrap();
    pub static ref COMP_REPLACE_XRANGES_LOOSE: Regex = Regex::new(r"^((?:<|>)?=?)\s*[v=\s]*([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:\.([0-9]+|x|X|\*)(?:(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*))*)))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)?)?$").unwrap();

    pub static ref COMP_REPLACE_STARS: Regex = Regex::new(r"(<|>)?=?\s*\*").unwrap();

    pub static ref COMPARATOR: Regex = Regex::new(r"^((?:<|>)?=?)\s*(v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)$|^$").unwrap();
    pub static ref COMPARATOR_LOOSE: Regex = Regex::new(r"^((?:<|>)?=?)\s*([v=\s]*([0-9]+)\.([0-9]+)\.([0-9]+)(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?)$|^$").unwrap();

    pub static ref VERSION: Regex = Regex::new(r"^v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][a-zA-Z0-9-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$").unwrap();
    pub static ref VERSION_LOOSE: Regex = Regex::new(r"^[v=\s]*([0-9]+)\.([0-9]+)\.([0-9]+)(?:-?((?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*)(?:\.(?:[0-9]+|\d*[a-zA-Z-][a-zA-Z0-9-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$").unwrap();

    pub static ref CLEAN_VERSION: Regex = Regex::new(r"^[=v]+").unwrap();
}
