#![feature(box_syntax)]

extern crate r7rs;

use r7rs::LispValue::*;
use r7rs::expression;

macro_rules! test_evaluation {
    ($name:ident, $($inp:expr => $out:expr),+) => (
        #[test]
        fn $name() {
            $(assert_eq!(expression(&$inp).unwrap().eval(), Ok($out));)+
        }
    );
}

test_evaluation!(basics,
    "(+ 1 1)" => Number(2),
    "(/ (+ 4 2) 2)" => Number(3),
    "(- 3 -1)" => Number(4),
    "(* six six)" => Number(36)
);
