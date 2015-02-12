#![feature(plugin, core, io)]
#[plugin] extern crate peg_syntax_ext;

mod lisp_value;

use std::old_io::stdin;
use scheme::expression;

peg_file! scheme("scheme.rustpeg");

fn read() -> String {
    stdin().read_line().unwrap().trim().to_string()
}

fn main() {
    loop {
        match expression(&read()) {
            Ok(ast)  => println!("{:?}\n=> {}", ast, ast),
            Err(err) => println!("{}", err)
        }
    }
}
