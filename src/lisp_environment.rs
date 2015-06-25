use std;
use std::rc::Rc;
use std::collections::HashMap;
use std::default::Default;
use super::lisp_value::{LispValue, LispResult, LispNum, PrimitiveFunction};

type LispVtable = HashMap<String, LispValue>;

macro_rules! lisp_funcs {
    ($($name:expr => $definition:expr),+ $(,)*) => ({
        let mut env: LispVtable = HashMap::new();
        $(
            let name = $name;
            env.insert(name.to_string(), LispValue::PrimitiveFunction(
                PrimitiveFunction::new(name, Rc::new($definition))
            ));
        )+
        env
    });
}

pub struct LispEnvironment {
    vtable: LispVtable,
}

impl LispEnvironment {
    pub fn call(&self, list: &[LispValue]) -> LispResult {
        match list {
            [LispValue::Atom(ref f), args..] => {
                if "quote".to_string() == *f { return Ok(args[0].clone()) };
                match self.vtable.get(f) {
                    Some(&LispValue::PrimitiveFunction(ref f)) => (f.clone().func)(&try!(self.eval_args(args))),
                    Some(&ref x) => Ok(x.clone()),
                    None => Err(format!("No such function: {}", f))
                }
            },
            [ref f, ..] => Err(format!("{} is not a function.", f)),
            [] => Ok(LispValue::List(vec![]))
        }
    }

    pub fn get(&self, identifier: &str) -> LispResult {
        match self.vtable.get(identifier) {
            Some(val) => Ok(val.clone()),
            None      => Err("Undefined variable!".into())
        }
    }

    fn eval_args(&self, args: &[LispValue]) ->  Result<Vec<LispValue>, String> {
        args.iter().map(|arg| arg.eval_in(self)).collect()
    }
}

impl Default for LispEnvironment {
    fn default() -> LispEnvironment {
        let mut vtable = lisp_funcs!(
            "+" => |args| numeric_op(args, 0, &|a, e| a + e),
            "-" => |args| numeric_op(args, 0, &|a, e| a - e),
            "*" => |args| numeric_op(args, 1, &|a, e| a * e),
            "/" => div,
        );
        vtable.insert("six".into(), LispValue::Number(6));
        LispEnvironment {vtable: vtable}
    }
}

fn numeric_op(operands: &[LispValue],
              fallback: LispNum,
              fold: &Fn(LispNum, LispNum) -> LispNum) -> LispResult {
    let mut numbers = operands.iter().map(assert_numericality);
    let initial = try!(numbers.next().unwrap_or(Ok(fallback)));
    if numbers.len() == 0 { return Ok(LispValue::Number(fold(fallback, initial))); }
    std::result::fold(numbers, initial, |a, e| fold(a, e)).map(LispValue::Number)
}

fn div(operands: &[LispValue]) -> LispResult {
    let numbers: Vec<LispNum> = try!(operands.iter().map(assert_numericality).collect());
    match &numbers[..] {
        []  => Err("Not enough arguments.".to_string()),
        [0] => Err("Cannot divide by zero.".to_string()),
        [n] => Ok(LispValue::Number(1 / n)),
        [0, ..] => Ok(LispValue::Number(0)),
        [n, rest..] => {
            let numbers = rest.iter().map(|item| match *item {
                0 => Err("Cannot divide by zero.".to_string()),
                x => Ok(x),
            });
            std::result::fold(numbers, n, |a, e| a / e).map(LispValue::Number)
        }
    }
}

fn assert_numericality(item: &LispValue) -> Result<LispNum, String> {
    match *item {
        LispValue::Number(n) => Ok(n),
        _ => Err(format!("Non-numeric operand: {}", item)),
    }
}
