extern crate r7rs;

use std::str::FromStr;
use r7rs::{LispNum, LispValue};
use r7rs::LispValue::*;

pub fn number(i: i32) -> LispValue {
    Number(LispNum::from_str(&i.to_string()).unwrap())
}

pub fn atom(ident: &str) -> LispValue {
    Atom(ident.into())
}

pub fn string(string: &str) -> LispValue {
    String(string.into())
}
