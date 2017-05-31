use std::rc::Rc;
use std::collections::HashMap;
use std::default::Default;

use num::{Zero, One};
use super::lisp_value::{LispValue, LispResult, LispNum, LispFunction, PrimitiveFunction};

type LispVtable = HashMap<String, LispValue>;

macro_rules! varargs {
    (nil) => (None);
    ($w:ident) => (Some(stringify!($w).into()));
}

macro_rules! lisp_funcs {
    ($($name:expr => [$($arg:ident),*], $varargs:ident, $definition:expr);+ $(;)*) => ({
        let mut env: LispVtable = HashMap::new();
        $(
            let name = $name;
            env.insert(name.into(), LispValue::PrimitiveFunction(
                PrimitiveFunction::new(
                    name,
                    &[$(stringify!($arg).into()),*],
                    varargs!($varargs),
                    Rc::new($definition))
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
        let result = match *list {
            [LispValue::Atom(ref f), ref args..] => {
                match &f[..] {
                    "define" => match *args {
                        [LispValue::List(ref args), ref body..] => {
                            match args[..] {
                                [LispValue::Atom(ref name), ref args..] => {
                                    let func = LispValue::Function(
                                        LispFunction::new(&new_world, args, body));
                                    new_world.set(name, func.clone());
                                    Ok(func)
                                },
                                _ => Err("Invalid function definition".into())
                            }
                        },
                        [LispValue::DottedList(ref args, ref varargs), ref body..] => {
                            match args[..] {
                                [LispValue::Atom(ref name), ref args..] => {
                                    let func = LispValue::Function(LispFunction::new_with_varargs(
                                        &new_world, args, *varargs.clone(), body));
                                    new_world.set(name, func.clone());
                                    Ok(func)
                                },
                                _ => Err("Invalid varargs function definition".into())
                            }
                        },
                        [LispValue::Atom(ref name), ref value] => {
                            let val = match *value {
                                LispValue::Atom(ref x) => {
                                    let y = self.get(x);
                                    if let Ok(z) = y {
                                        z
                                    } else {
                                        return (y, new_world);
                                    }
                                },
                                LispValue::List(ref f) |
                                LispValue::DottedList(ref f, _) => {
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
                    "set!" => match *args {
                        [LispValue::Atom(ref name), ref value] => {
                            if new_world.defined(name) {
                                let val = match *value {
                                    LispValue::Atom(ref x) => {
                                        let y = self.get(x);
                                        if let Ok(z) = y {
                                            z
                                        } else {
                                            return (y, new_world);
                                        }
                                    },
                                    LispValue::List(ref f) |
                                    LispValue::DottedList(ref f, _) => {
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
                    "lambda" => match *args {
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
                    "if" => match *args {
                        [ref predicate, ref consequent, ref alternate] => {
                            let (result, tmp_world) = predicate.eval_in(self);
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
                        Some(&LispValue::PrimitiveFunction(ref f)) => {
                            f.check_arity(args)
                                .and_then(|args| self.eval_args(&args))
                                .and_then(|args| f.call(&new_world, &args))
                        },
                        Some(&LispValue::Function(ref f)) =>
                            f.check_arity(args)
                                .and_then(|args| self.eval_args(&args))
                                .and_then(|args| f.call(&new_world, &args)),
                        Some(&ref x) => Err(format!("No such function: {}", x)),
                        None => Err(format!("No such function: {}", f))
                    }
                }
            },
            [LispValue::PrimitiveFunction(ref f), ref args..] =>
                f.check_arity(args)
                    .and_then(|args| self.eval_args(&args))
                    .and_then(|args| f.call(&new_world, &args)),
            [LispValue::Function(ref f), ref args..] =>
                f.check_arity(args)
                    .and_then(|args| self.eval_args(&args))
                    .and_then(|args| f.call(&new_world, &args)),
            [LispValue::List(ref f), ref args..] |
            [LispValue::DottedList(ref f, _), ref args..] => {
                let val = new_world.call(f).0;
                if let Ok(value) = val {
                    let mut new_list = vec![value];
                    new_list.extend_from_slice(args);
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

    fn eval_args(&self, args: &[LispValue]) -> Result<Vec<LispValue>, String> {
        args.iter().map(|arg| arg.eval_in(self).0).collect()
    }
}

impl Default for LispEnvironment {
    fn default() -> LispEnvironment {
        let vtable = lisp_funcs!(
            "+"    => [], xs, |args| numeric_op(args, &LispNum::zero(), &|a, e| a + e);
            "-"    => [], xs, |args| numeric_op(args, &LispNum::zero(), &|a, e| a - e);
            "*"    => [], xs, |args| numeric_op(args, &LispNum::one(), &|a, e| a * e);
            ">"    => [], xs, |args| comparison_op(args, &|a, e| a > e);
            "<"    => [], xs, |args| comparison_op(args, &|a, e| a < e);
            ">="   => [], xs, |args| comparison_op(args, &|a, e| a >= e);
            "<="   => [], xs, |args| comparison_op(args, &|a, e| a <= e);
            "="    => [], xs, |args| comparison_op(args, &|a, e| a == e);
            "/"    => [x], xs, div;
            "cons" => [car, cdr], nil, cons;
        );
        LispEnvironment {vtable: vtable}
    }
}

fn numeric_op(operands: &[LispValue],
              fallback: &LispNum,
              fold: &Fn(LispNum, LispNum) -> LispNum) -> LispResult {
    let mut numbers = operands.iter().map(assert_numericality);
    let initial = try!(numbers.next().unwrap_or_else(|| Ok(fallback.clone())));
    if numbers.len() == 0 { return Ok(LispValue::Number(fold(fallback.clone(), initial))); }
    result_fold(numbers, initial, fold).map(LispValue::Number)
}

fn div(operands: &[LispValue]) -> LispResult {
    let numbers: Vec<LispNum> = try!(operands.iter().map(assert_numericality).collect());
    let zero: LispNum = LispNum::zero();

    match numbers[..] {
        []  => Err("Not enough arguments.".into()),
        [ref n] => if n.is_zero() {
                Err("Cannot divide by zero.".into())
            } else {
                Ok(LispValue::Number(LispNum::one() / n))
            },
        [ref n, ref rest..] => {
            if n.is_zero() { return Ok(LispValue::Number(zero)) }
            let numbers = rest.iter().map(|item|
                if item.is_zero() {
                    Err("Cannot divide by zero.".into())
                } else {
                    Ok(item)
                }
            );
            result_fold(numbers, n.clone(), |a, e| a / e).map(LispValue::Number)
        }
    }
}

fn comparison_op(operands: &[LispValue],
               fold: &Fn(&LispNum, &LispNum) -> bool) -> LispResult {
    let numbers: Vec<LispNum> = try!(operands.iter().map(assert_numericality).collect());
    if numbers.len() < 2 { return Err("Need at least two args to compare".into()); }
    let val = numbers.iter().zip(&numbers[1..]).all(|(a, b)| fold(a, b));

    Ok(LispValue::Boolean(val))
}

fn assert_numericality(item: &LispValue) -> Result<LispNum, String> {
    match *item {
        LispValue::Number(ref n) => Ok(n.clone()),
        _ => Err(format!("Non-numeric operand: {}", item)),
    }
}

fn cons(operands: &[LispValue]) -> LispResult {
    match *operands {
        [ref elt, LispValue::List(ref xs)] => {
            let mut new_list = xs.clone();
            new_list.insert(0, elt.clone());
            Ok(LispValue::List(new_list))
        },
        [ref elt, LispValue::DottedList(ref xs, ref xlast)] => {
            let mut new_list = xs.clone();
            new_list.insert(0, elt.clone());
            Ok(LispValue::DottedList(new_list, xlast.clone()))
        },
        [ref elt1, ref elt2] => Ok(LispValue::DottedList(vec![elt1.clone()], Box::new(elt2.clone()))),
        _ => unreachable!()
    }
}

fn result_fold<T,
            V,
            E,
            F: FnMut(V, T) -> V,
            Iter: Iterator<Item=Result<T, E>>>(
            iterator: Iter,
            mut init: V,
            mut f: F)
            -> Result<V, E> {
    for t in iterator {
        match t {
            Ok(v) => init = f(init, v),
            Err(u) => return Err(u)
        }
    }
    Ok(init)
}
