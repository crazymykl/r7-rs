use std;
use std::collections::HashMap;
use std::default::Default;
use super::lisp_value::{LispValue, LispResult, LispNum};

type LispFunction = Fn(&[LispValue]) -> LispResult;

pub struct LispEnvironment {
    vtable: HashMap<LispValue, Box<LispFunction>>,
}

impl LispEnvironment {
    pub fn call(&self, list: &[LispValue]) -> LispResult {
        match list {
            [ref f @ LispValue::Atom(_), args..] => {
                if LispValue::Atom("quote".to_string()) == *f { return Ok(args[0].clone()) };
                match self.vtable.get(f) {
                    Some(f) => f(&try!(eval_args(args, self))),
                    None => Err(format!("No such function: {}", f))
                }
            },
            [ref f, ..] => Err(format!("{} is not a function.", f)),
            [] => Ok(LispValue::List(vec![]))
        }
    }
}

impl Default for LispEnvironment {
    fn default() -> LispEnvironment {
        let mut env : HashMap<LispValue, Box<LispFunction>> = HashMap::new();
        env.insert(LispValue::Atom("+".to_string()), box add);
        LispEnvironment { vtable: env }
    }
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

fn eval_args(args: &[LispValue], env: &LispEnvironment) -> Result<Vec<LispValue>, String> {
    args.iter().map(|arg| arg.eval(env)).collect()
}
