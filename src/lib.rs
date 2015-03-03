#![feature(plugin, core, old_io, int_uint, box_syntax)]
#![plugin(peg_syntax_ext)]

mod lisp_value;

use std::old_io::stdin;
pub use lisp_value::{LispValue, baseline};
pub use scheme::expression;

peg_file! scheme("scheme.rustpeg");

fn read() -> String {
    stdin().read_line().unwrap().trim().to_string()
}

pub fn main() {
    loop {
        match expression(&read()) {
            Ok(ast)  => println!("{:?}\n{}", ast, ast.eval(&baseline())),
            Err(err) => println!("{}", err)
        }
    }
}
