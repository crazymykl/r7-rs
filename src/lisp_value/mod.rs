use std::fmt;
use std::default::Default;

pub mod primitive_function;

use super::lisp_environment::LispEnvironment;
pub use self::primitive_function::PrimitiveFunction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LispValue {
    Atom(String),
    List(Vec<LispValue>),
    DottedList(Vec<LispValue>, Box<LispValue>),
    Number(LispNum),
    String(String),
    PrimitiveFunction(PrimitiveFunction),
    Boolean(bool)
}

pub type LispNum = i64;
pub type LispResult = Result<LispValue, String>;

impl LispValue {
    pub fn quote(expression: LispValue) -> LispValue {
        LispValue::List(vec![LispValue::Atom("quote".to_string()), expression])
    }

    pub fn eval(&self) -> LispResult {
        self.eval_in(&LispEnvironment::default())
    }

    pub fn eval_in(&self, world: &LispEnvironment) -> LispResult {
        match *self {
            LispValue::List(ref v) |
            LispValue::DottedList(ref v, _) => world.call(v),
            LispValue::Atom(ref v) => world.get(v),
            _ => Ok(self.clone())
        }
    }
}

impl fmt::Display for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match *self {
            LispValue::Atom(ref x) => x.to_string(),
            LispValue::List(ref x) => format!("({})", format_list(x)),
            LispValue::DottedList(ref x, ref y) => format!("({} . {})", format_list(x), y),
            LispValue::Number(x) => x.to_string(),
            LispValue::String(ref x) => format!("\"{}\"", x),
            LispValue::PrimitiveFunction(_) => "<primitive function>".to_string(),
            LispValue::Boolean(true) => "#t".to_string(),
            LispValue::Boolean(false) => "#f".to_string(),
        };
        write!(f, "{}", string)
    }
}

fn format_list(list: &Vec<LispValue>) -> String {
    list.iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .connect(" ")
}