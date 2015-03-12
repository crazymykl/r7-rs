#![feature(plugin, core, io, box_syntax, unicode, collections)]
#![plugin(peg_syntax_ext)]

mod lisp_value;
mod lisp_environment;

use std::io::{stdin, BufRead};

pub use lisp_value::{LispValue, LispNum};
pub use scheme::expression;

peg_file! scheme("scheme.rustpeg");

fn read() -> String {
    let (mut inp, stdin) = ("".to_string(), stdin());
    stdin.lock().read_line(&mut inp).unwrap();
    inp.trim().to_string()
}

pub fn main() {
    loop {
        match expression(&read()) {
            Ok(ast)  => println!("{:?}\n{}", ast, ast.eval()),
            Err(err) => println!("{}", err),
        }
    }
}
