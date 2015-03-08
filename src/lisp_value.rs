use std::{self, fmt};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LispValue {
    Atom(String),
    List(Vec<LispValue>),
    DottedList(Vec<LispValue>, Box<LispValue>),
    Number(LispNum),
    String(String),
    Boolean(bool)
}

pub type LispNum = i64;
pub type LispResult = Result<LispValue, String>;
type LispFunction = Fn(&[LispValue]) -> LispResult;
pub type LispEnvironment = HashMap<LispValue, Box<LispFunction>>;

pub fn baseline() -> LispEnvironment {
    let mut env: LispEnvironment = HashMap::new();
    env.insert(LispValue::Atom("+".to_string()), box add);
    env
}

fn add(operands: &[LispValue]) -> LispResult {
    let numbers = operands.iter().map(assert_numericality);
    std::result::fold(numbers, 0, |a, e| a + e).map(LispValue::Number)
}

fn assert_numericality(item: &LispValue) -> Result<LispNum, String> {
    match *item {
        LispValue::Number(n) => Ok(n),
        _ => Err(format!("Non-numeric operand: {}", item)),
    }
}

impl LispValue {
    pub fn quote(expression: LispValue) -> LispValue {
        LispValue::List(vec![LispValue::Atom("quote".to_string()), expression])
    }

    pub fn eval(&self, world: &LispEnvironment) -> LispResult {
        match *self {
            LispValue::List(ref v) |
            LispValue::DottedList(ref v, _) => function(v, &world),
            _ => Ok(self.clone())
        }
    }
}

fn function(list: &[LispValue], world: &LispEnvironment) -> LispResult {
    match list {
        [ref f @ LispValue::Atom(_), args..] => {
            if LispValue::Atom("quote".to_string()) == *f { return Ok(args[0].clone()) };
            match world.get(f) {
                Some(f) => f(&try!(eval_args(args, world))),
                None => Err(format!("No such fuction: {}", f))
            }
        },
        [ref f, ..] => Err(format!("{} is not a fuction.", f)),
        [] => Ok(LispValue::List(vec![]))
    }
}

fn eval_args(args: &[LispValue], env: &LispEnvironment) -> Result<Vec<LispValue>, String> {
    args.iter().map(|arg| arg.eval(env)).collect()
}

impl std::fmt::Display for LispResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Ok(ref item) => write!(f, "=> {}", item),
            Err(ref msg) => write!(f, "ERR: {}", msg)
        }
    }
}

impl std::fmt::Display for Vec<LispValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elts = self.iter().map(|i| i.to_string()).collect::<Vec<String>>();
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
