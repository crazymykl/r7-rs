#![feature(plugin, core, io)]
#![plugin(peg_syntax_ext)]

mod lisp_value;

use std::old_io::stdin;
pub use lisp_value::LispValue;
pub use scheme::expression;

peg_file! scheme("scheme.rustpeg");

fn read() -> String {
    stdin().read_line().unwrap().trim().to_string()
}

pub fn main() {
    loop {
        match expression(&read()) {
            Ok(ast)  => println!("{:?}\n=> {}", ast, ast),
            Err(err) => println!("{}", err)
        }
    }
}
