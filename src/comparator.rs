use super::{operator::Operator, version::Version};

use failure::{err_msg, Error};

pub struct Comparator {
    pub operator: Operator,
    pub version: Version,
}

impl Comparator {
    pub fn new<S: Into<String>>(version: S) -> Result<Comparator, Error> {
        Err(err_msg(""))
    }
}
