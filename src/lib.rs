#![allow(dead_code)]

extern crate regex;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

mod comparator;
mod expressions;
mod operator;
mod range;
mod util;
mod version;

#[cfg(test)]
mod tests {}
