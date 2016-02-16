#![feature(box_syntax)]

extern crate r7rs;

mod shared;

use shared::*;
use r7rs::{LispEnvironment, expression};
use r7rs::LispValue::*;

macro_rules! test_evaluation {
    ($name:ident, $($($inp:expr),+ => $out:expr),+) => (
        #[test]
        #[allow(unused_variables)]
        fn $name() {
            $(
                let world = LispEnvironment::default();
                $(let (result, world) = expression(&$inp).unwrap().eval_in(&world);)+
                assert_eq!(result, Ok($out));
            )+
        }
    );
}

test_evaluation!(basics,
    "(+ 1 1)" => number(2),
    "(/ (+ 4 2) 2)" => number(3),
    "(- 3 -1)" => number(4)
);

test_evaluation!(assignment,
    "(define six 6)", "(* six six)" => number(36),
    "(define foo 3)", "(define bar foo)", "bar" => number(3),
    "(define mew \"cat\")", "(set! mew \"kitten\")", "mew" => string("kitten")
);

test_evaluation!(conditionals,
    "(if (= 2 3) \"yay\" 'boo)" => atom("boo"),
    "(if (= 2 2 2 2) \"yay\" 'boo)" => string("yay")
);

test_evaluation!(functions,
    "(define (list . xs) xs)", "(list 1 2)" => List(vec![number(1), number(2)]),
    "((if 2 - +) 2 1)" => number(1),
    "(define (counter inc) (lambda (x) (define inc (+ x inc)) inc))",
        "(define my-count (counter 5))",
        "(my-count 4)",
        "(my-count 4)" => number(13),
    "(define (fact n) (if (= n 0) 1 (* n (fact (- n 1)))))", "(fact 6)" => number(720)
);
