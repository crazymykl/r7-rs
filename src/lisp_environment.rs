use std;
use std::collections::HashMap;
use std::default::Default;
use super::lisp_value::{LispValue, LispResult, LispNum};

type LispFunction = Fn(&[LispValue]) -> LispResult;
type LispVtable = HashMap<LispValue, Box<LispFunction>>;

macro_rules! lisp_funcs {
    ($($name:expr => $definition:expr),+ $(,)*) => ({
        let mut env: LispVtable = HashMap::new();
        $(env.insert(LispValue::Atom($name.to_string()),  box $definition);)+
        env
    });
}

pub struct LispEnvironment {
    vtable: LispVtable,
}

impl LispEnvironment {
    pub fn call(&self, list: &[LispValue]) -> LispResult {
        match list {
            [ref f @ LispValue::Atom(_), args..] => {
                if LispValue::Atom("quote".to_string()) == *f { return Ok(args[0].clone()) };
                match self.vtable.get(f) {
                    Some(f) => f(&try!(self.eval_args(args))),
                    None => Err(format!("No such function: {}", f))
                }
            },
            [ref f, ..] => Err(format!("{} is not a function.", f)),
            [] => Ok(LispValue::List(vec![]))
        }
    }

    fn eval_args(&self, args: &[LispValue]) ->  Result<Vec<LispValue>, String> {
        args.iter().map(|arg| arg.eval_in(self)).collect()
    }
}

impl Default for LispEnvironment {
    fn default() -> LispEnvironment {
        let vtable = lisp_funcs!(
            "+" => |args| numeric_op(args, &|a, e| a + e),
            "-" => |args| match args {
                [LispValue::Number(n)] => Ok(LispValue::Number(-n)),
                _ => numeric_op(args, &|a, e| a - e),
            },
            "*" => |args| numeric_op(args, &|a, e| a * e),
            "/" => |args| numeric_op(args, &|a, e| a / e)
        );
        LispEnvironment {vtable: vtable}
    }
}

fn numeric_op(operands: &[LispValue],
              fold: &Fn(LispNum, LispNum) -> LispNum) -> LispResult {
    let mut numbers = operands.iter().map(assert_numericality);
    let initial = try!(numbers.next().unwrap());
    std::result::fold(numbers, initial, |a, e| fold(a, e)).map(LispValue::Number)
}

fn assert_numericality(item: &LispValue) -> Result<LispNum, String> {
    match *item {
        LispValue::Number(n) => Ok(n),
        _ => Err(format!("Non-numeric operand: {}", item)),
    }
}
