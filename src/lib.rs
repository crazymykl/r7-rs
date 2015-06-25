#![feature(plugin, box_syntax, slice_patterns, unboxed_closures, result_fold)]
#![plugin(peg_syntax_ext)]

mod lisp_value;
mod lisp_environment;

use std::io::{stdin, BufRead};

pub use lisp_value::{LispValue, LispNum};
pub use lisp_environment::LispEnvironment;
pub use scheme::expression;

peg_file! scheme("scheme.rustpeg");

fn read() -> String {
    let (mut inp, stdin) = ("".to_string(), stdin());
    stdin.lock().read_line(&mut inp).unwrap();
    inp.trim().to_string()
}

pub fn main() {
    let mut world = LispEnvironment::default();
    loop {
        match expression(&read()) {
            Ok(ast)  => {
                let (result, new_world) = ast.eval_in(&world);
                world = new_world;
                println!("{:?}\n{:?}\n{:?}", ast, result, world)
            },
            Err(err) => println!("{:?}", err),
        }
    }
}
