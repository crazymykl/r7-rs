use std::{self, fmt};
use std::default::Default;

use super::lisp_environment::LispEnvironment;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LispValue {
    Atom(String),
    List(LispList),
    DottedList(LispList, Box<LispValue>),
    Number(LispNum),
    String(String),
    Boolean(bool)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LispList(pub Vec<LispValue>);

pub type LispNum = i64;
pub type LispResult = Result<LispValue, String>;

impl LispValue {
    pub fn quote(expression: LispValue) -> LispValue {
        LispValue::List(LispList(vec![LispValue::Atom("quote".to_string()), expression]))
    }

    pub fn eval(&self) -> LispResult {
        self.eval_in(&LispEnvironment::default())
    }

    pub fn eval_in(&self, world: &LispEnvironment) -> LispResult {
        match *self {
            LispValue::List(ref v) |
            LispValue::DottedList(ref v, _) => world.call(&v.0),
            _ => Ok(self.clone())
        }
    }
}

impl std::fmt::Display for LispList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elts = self.0.iter().map(|i| i.to_string()).collect::<Vec<String>>();
        write!(f, "{}", elts.connect(" "))
    }
}

impl std::fmt::Display for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match *self {
            LispValue::Atom(ref x) => x.to_string(),
            LispValue::List(ref x) => format!("({})", x),
            LispValue::DottedList(ref x, ref y) => format!("({} . {})", x, y),
            LispValue::Number(x) => x.to_string(),
            LispValue::String(ref x) => format!("\"{}\"", x),
            LispValue::Boolean(true) => "#t".to_string(),
            LispValue::Boolean(false) => "#f".to_string(),
        };
        write!(f, "{}", string)
    }
}
