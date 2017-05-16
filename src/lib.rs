#![feature(plugin, box_syntax, slice_patterns, unboxed_closures)]
#![plugin(peg_syntax_ext)]

extern crate num;
extern crate rustyline;

mod lisp_value;
mod lisp_environment;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub use lisp_value::{LispValue, LispNum};
pub use lisp_environment::LispEnvironment;
pub use scheme::{expression, completeInput};

const HISTORY_FILE: &'static str = "history.txt";

peg_file! scheme("scheme.rustpeg");

pub fn main() {
    let mut world = LispEnvironment::default();
    let mut rl = Editor::<()>::new();

    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    'outer: loop {
        match rl.readline(">>> ") {
            Ok(mut line) => {
                if line.trim().is_empty() { continue }
                rl.add_history_entry(&line);
                while incomplete(&line) {
                    match rl.readline(">>* ") {
                        Ok(additional_line) => {
                            rl.add_history_entry(&additional_line);
                            line.push_str("\n");
                            line.push_str(&additional_line) },
                        Err(ReadlineError::Interrupted) |
                            Err(ReadlineError::Eof) => continue 'outer,
                        Err(err) => {
                            println!("Error: {:?}", err);
                            break 'outer
                        }
                    }
                }
                if let Some(new_world) = evaluate(line.trim(), &world) { world = new_world; }
            },
            Err(ReadlineError::Interrupted) => {},
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history(HISTORY_FILE).unwrap();
}

fn incomplete(line: &str) -> bool {
    completeInput(line).is_err()
}

fn evaluate(input: &str, world: &LispEnvironment) -> Option<LispEnvironment> {
    match expression(input) {
        Ok(ast)  => {
            let (result, new_world) = ast.eval_in(world);
            if cfg!(feature = "show_ast") { println!("{:?}", ast); }
            match result {
                Ok(cool) => {
                    println!("<<< {}", cool);
                    return Some(new_world);
                }
                Err(or)  => println!("ERR {}", or)
            }
        },
        Err(err) => println!("{:?}", err),
    }
    None
}
