use std::fmt;
use std::default::Default;
use num::rational;

pub mod primitive_function;
pub mod lisp_function;

use super::lisp_environment::LispEnvironment;
pub use self::primitive_function::PrimitiveFunction;
pub use self::lisp_function::LispFunction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LispValue {
    Atom(String),
    List(Vec<LispValue>),
    DottedList(Vec<LispValue>, Box<LispValue>),
    Number(LispNum),
    String(String),
    PrimitiveFunction(PrimitiveFunction),
    Function(LispFunction),
    Boolean(bool)
}

pub type LispNum = rational::BigRational;
pub type LispResult = Result<LispValue, String>;

impl LispValue {
    pub fn quote(expression: LispValue) -> LispValue {
        LispValue::List(vec![LispValue::Atom("quote".into()), expression])
    }

    pub fn eval(&self) -> LispResult {
        self.eval_in(&LispEnvironment::default()).0
    }

    pub fn eval_in(&self, world: &LispEnvironment) -> (LispResult, LispEnvironment) {
        match *self {
            LispValue::List(ref v) |
            LispValue::DottedList(ref v, _) => world.call(v),
            LispValue::Atom(ref v) => (world.get(v), world.clone()),
            _ => (Ok(self.clone()), world.clone())
        }
    }
}

impl fmt::Display for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match *self {
            LispValue::Atom(ref x) => x.clone(),
            LispValue::List(ref x) => format!("({})", format_list(x)),
            LispValue::DottedList(ref x, ref y) => format!("({} . {})", format_list(x), y),
            LispValue::Number(ref x) => x.to_string(),
            LispValue::String(ref x) => format!("\"{}\"", x),
            LispValue::PrimitiveFunction(ref f) => format!("<primitive function:({})>", f.arg_list()),
            LispValue::Function(ref f) => format!("<function:({})>", f.arg_list()),
            LispValue::Boolean(true) => "#t".into(),
            LispValue::Boolean(false) => "#f".into(),
        };
        write!(f, "{}", string)
    }
}

fn format_list(list: &[LispValue]) -> String {
    list.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
