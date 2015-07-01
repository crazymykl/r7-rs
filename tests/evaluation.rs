#![feature(box_syntax)]

extern crate r7rs;

use r7rs::LispValue::*;
use r7rs::LispEnvironment;
use r7rs::expression;

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
    "(+ 1 1)" => Number(2),
    "(/ (+ 4 2) 2)" => Number(3),
    "(- 3 -1)" => Number(4)
);

test_evaluation!(assignment,
    "(define six 6)", "(* six six)" => Number(36),
    "(define foo 3)", "(define bar foo)", "bar" => Number(3),
    "(define mew \"cat\")", "(set! mew \"kitten\")", "mew" => String("kitten".into())
);

test_evaluation!(conditionals,
    "(if (= 2 3) \"yay\" \"boo\")" => String("boo".into()),
    "(if (= 2 2 2 2) \"yay\" \"boo\")" => String("yay".into())
);

test_evaluation!(functions,
    "(define (list . xs) xs)", "(list 1 2)" => List(vec![Number(1), Number(2)]),
    "((if 2 - +) 2 1)" => Number(1),
    "(define (counter inc) (lambda (x) (define inc (+ x inc)) inc))",
        "(define my-count (counter 5))",
        "(my-count 4)",
        "(my-count 4)" => Number(13),
    "(define (fact n) (if (= n 0) 1 (* n (fact (- n 1)))))", "(fact 6)" => Number(720)
);
