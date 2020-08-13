use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Operator {
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Ne,
    StrictEq,
    StrictNe,

    Empty,
}

impl Operator {
    pub fn new(s: &str) -> Operator {
        match s {
            ">" => Operator::Gt,
            "<" => Operator::Lt,
            ">=" => Operator::Gte,
            "<=" => Operator::Lte,
            "=" => Operator::Eq,
            "==" => Operator::Eq,
            "!=" => Operator::Ne,
            "===" => Operator::StrictEq,
            "!==" => Operator::StrictNe,
            _ => Operator::Empty,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Gte => ">=",
            Operator::Lte => "<=",
            Operator::Eq => "",
            Operator::Ne => "!=",
            Operator::StrictEq => "===",
            Operator::StrictNe => "!==",
            Operator::Empty => "",
        };

        write!(f, "{}", s)
    }
}
