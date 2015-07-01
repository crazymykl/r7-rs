use std;
use std::rc::Rc;
use std::collections::HashMap;
use std::default::Default;
use super::lisp_value::{LispValue, LispResult, LispNum, LispFunction, PrimitiveFunction};

type LispVtable = HashMap<String, LispValue>;

macro_rules! lisp_funcs {
    ($($name:expr => $definition:expr),+ $(,)*) => ({
        let mut env: LispVtable = HashMap::new();
        $(
            let name = $name;
            env.insert(name.into(), LispValue::PrimitiveFunction(
                PrimitiveFunction::new(name, Rc::new($definition))
            ));
        )+
        env
    });
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LispEnvironment {
    vtable: LispVtable,
}

impl LispEnvironment {
    pub fn call(&self, list: &[LispValue]) -> (LispResult, LispEnvironment) {
        let mut new_world = self.clone();
        let result = match list {
            [LispValue::Atom(ref f), args..] => {
                match &f[..] {
                    "define" => match args {
                        [LispValue::List(ref args), ref body..] => {
                            match &args[..] {
                                [LispValue::Atom(ref name), args..] => {
                                    let func = LispValue::Function(
                                        LispFunction::new(&new_world, args, body));
                                    new_world.set(name, func.clone());
                                    Ok(func)
                                },
                                _ => Err("Invalid function definition".into())
                            }
                        },
                        [LispValue::DottedList(ref args, ref varargs), ref body..] => {
                            match &args[..] {
                                [LispValue::Atom(ref name), args..] => {
                                    let func = LispValue::Function(LispFunction::new_with_varargs(
                                        &new_world, args, *varargs.clone(), body));
                                    new_world.set(name, func.clone());
                                    Ok(func)
                                },
                                _ => Err("Invalid varargs function definition".into())
                            }
                        },
                        [LispValue::Atom(ref name), ref value] => {
                            let val = match value {
                                &LispValue::Atom(ref x) => {
                                    let y = self.get(x);
                                    if let Ok(z) = y {
                                        z
                                    } else {
                                        return (y, new_world);
                                    }
                                },
                                &LispValue::List(ref f) |
                                &LispValue::DottedList(ref f, _) => {
                                    let y = self.call(f).0;
                                    if let Ok(z) = y {
                                        z
                                    } else {
                                        return (y, new_world);
                                    }
                                }
                                _ => value.clone()
                            };
                            new_world.set(name, val.clone());
                            Ok(val.clone())
                        },
                        _ => Err("Invalid definition".into())
                    },
                    "set!" => match args {
                        [LispValue::Atom(ref name), ref value] => {
                            if new_world.defined(name) {
                                let val = match value {
                                    &LispValue::Atom(ref x) => {
                                        let y = self.get(x);
                                        if let Ok(z) = y {
                                            z
                                        } else {
                                            return (y, new_world);
                                        }
                                    },
                                    &LispValue::List(ref f) |
                                    &LispValue::DottedList(ref f, _) => {
                                        let y = self.call(f).0;
                                        if let Ok(z) = y {
                                            z
                                        } else {
                                            return (y, new_world);
                                        }
                                    }
                                    _ => value.clone()
                                };
                                new_world.set(name, val.clone());
                                Ok(val.clone())
                            } else {
                                Err(format!("Undefined variable: '{}'", name))
                            }
                        },
                        _ => Err("Invalid set!".into())
                    },
                    "lambda" => match args {
                        [LispValue::List(ref args), ref body..] =>
                            Ok(LispValue::Function(LispFunction::new(
                                &new_world, args, body))),
                        [LispValue::DottedList(ref args, ref varargs), ref body..] =>
                            Ok(LispValue::Function(LispFunction::new_with_varargs(
                                &new_world, args, *varargs.clone(), body))),
                        [ref varargs, ref body..] =>
                            Ok(LispValue::Function(LispFunction::new_with_varargs(
                                &new_world, &[], varargs.clone(), body))),
                        _ => Err("Invalid lambda".into())
                    },
                    "quote" => Ok(args[0].clone()),
                    "if" => match args {
                        [ref predicate, ref consequent, ref alternate] => {
                            let (result, tmp_world) = predicate.eval_in(&self);
                            let branch = match result {
                                Ok(LispValue::Boolean(false)) => alternate,
                                Ok(_)                         => consequent,
                                Err(_)                        => return (result, tmp_world)
                            };

                            return branch.eval_in(&tmp_world);
                        },
                        _ => Err("Bad 'if'".into())
                    },
                    _ => match self.vtable.get(f) {
                        Some(&LispValue::PrimitiveFunction(ref f)) =>
                            self.eval_args(args).and_then(|args| f.call(&args)),
                        Some(&LispValue::Function(ref f)) =>
                            self.eval_args(args).and_then(|args| f.call(&new_world, &args)),
                        Some(&ref x) => Err(format!("No such function: {}", x)),
                        None => Err(format!("No such function: {}", f))
                    }
                }
            },
            [LispValue::PrimitiveFunction(ref f), args..] =>
                self.eval_args(args).and_then(|args| f.call(&args)),
            [LispValue::Function(ref f), args..] =>
                self.eval_args(args).and_then(|args| f.call(&new_world, &args)),
            [LispValue::List(ref f), args..] |
            [LispValue::DottedList(ref f, _), args..] => {
                let val = new_world.call(f).0;
                if let Ok(value) = val {
                    let mut new_list = vec![value];
                    new_list.push_all(args);
                    return new_world.call(&new_list);
                } else {
                    val
                }
            },
            [ref f, ..] => Err(format!("{} is not a function.", f)),
            [] => Ok(LispValue::List(vec![]))
        };
        (result, new_world)
    }

    pub fn eval_many(&self, forms: &[LispValue]) -> (LispResult, LispEnvironment) {
        let mut world = self.clone();
        let mut result = Ok(LispValue::List(vec![]));

        for form in forms {
            let (new_result, new_world) = form.eval_in(&world);
            world = new_world;
            if new_result.is_ok() {
                result = new_result;
            } else {
                return (new_result, world);
            }
        };

        (result, world)
    }

    pub fn get(&self, identifier: &str) -> LispResult {
        match self.vtable.get(identifier) {
            Some(val) => Ok(val.clone()),
            None      => Err(format!("Undefined variable: '{}'!", identifier))
        }
    }

    pub fn set(&mut self, name: &str, value: LispValue) {
        self.vtable.insert(name.into(), value);
    }

    pub fn merge(&self, other: &LispEnvironment) -> LispEnvironment {
        let mut new = self.clone();

        for (name, value) in other.clone().vtable {
            new.set(&name, value.clone());
        };

        new
    }

    fn defined(&self, name: &str) -> bool {
        self.vtable.contains_key(name.into())
    }

    fn eval_args(&self, args: &[LispValue]) ->  Result<Vec<LispValue>, String> {
        args.iter().map(|arg| arg.eval_in(self).0).collect()
    }
}

impl Default for LispEnvironment {
    fn default() -> LispEnvironment {
        let vtable = lisp_funcs!(
            "+" => |args| numeric_op(args, 0, &|a, e| a + e),
            "-" => |args| numeric_op(args, 0, &|a, e| a - e),
            "*" => |args| numeric_op(args, 1, &|a, e| a * e),
            "=" => equal,
            "/" => div,
        );
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
        []  => Err("Not enough arguments.".into()),
        [0] => Err("Cannot divide by zero.".into()),
        [n] => Ok(LispValue::Number(1 / n)),
        [0, ..] => Ok(LispValue::Number(0)),
        [n, rest..] => {
            let numbers = rest.iter().map(|item| match *item {
                0 => Err("Cannot divide by zero.".into()),
                x => Ok(x),
            });
            std::result::fold(numbers, n, |a, e| a / e).map(LispValue::Number)
        }
    }
}

fn equal(operands: &[LispValue]) -> LispResult {
    match operands {
        [ref first, ref second, ref rest..] =>
            Ok(LispValue::Boolean(first == second && rest.iter().all(|e| e == first))),
        _ => Err("Need at least two args to equality".into())
    }
}

fn assert_numericality(item: &LispValue) -> Result<LispNum, String> {
    match *item {
        LispValue::Number(n) => Ok(n),
        _ => Err(format!("Non-numeric operand: {}", item)),
    }
}
