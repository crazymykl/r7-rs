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
    "(- 3 -1)" => Number(4),
    "(define six 6)", "(* six six)" => Number(36),
    "(if (= 2 3) \"yay\" \"boo\")" => String("boo".into()),
    "(if (= 2 2 2 2) \"yay\" \"boo\")" => String("yay".into())
);
