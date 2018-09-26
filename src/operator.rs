use failure::{Error, Fail};
use std::fmt;

pub enum Operator {
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Ne,
    StrictEq,
    StrictNe,
}

impl Operator {
    pub fn new(s: String) -> Result<Operator, Error> {
        let op = match s.as_str() {
            ">" => Operator::Gt,
            "<" => Operator::Lt,
            ">=" => Operator::Gte,
            "<=" => Operator::Lte,
            "=" => Operator::Eq,
            "" => Operator::Eq,
            "==" => Operator::Eq,
            "!=" => Operator::Ne,
            "===" => Operator::StrictEq,
            "!==" => Operator::StrictNe,
            _ => return Err(OperatorError::Invalid(s).into()),
        };
        Ok(op)
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
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Fail)]
pub enum OperatorError {
    #[fail(display = "invalid operator {}", _0)]
    Invalid(String),
}
