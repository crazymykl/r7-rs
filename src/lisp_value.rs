use std::{self, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LispValue {
    Atom(String),
    List(Vec<LispValue>),
    DottedList(Vec<LispValue>, Box<LispValue>),
    Number(i64),
    String(String),
    Boolean(bool)
}

impl LispValue {
  pub fn quote(expression: LispValue) -> LispValue {
    LispValue::List(vec![LispValue::Atom("quote".to_string()), expression])
  }
}

impl std::fmt::Display for Vec<LispValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elts: Vec<String> = self.iter().map(|i| i.to_string()).collect();
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
