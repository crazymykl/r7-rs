#![feature(box_syntax)]

extern crate r7rs;

use r7rs::LispValue;
use r7rs::LispValue::*;
use r7rs::expression;

macro_rules! test_parsing {
    ($name:ident, $($inp:expr => $out:expr),+) => (
        #[test]
        fn $name() {
            $(assert_eq!(expression(&$inp), Ok($out));)+
        }
    );
}

fn atom(ident: &str) -> LispValue {
    Atom(ident.to_string())
}

fn string(string: &str) -> LispValue {
    String(string.to_string())
}

test_parsing!(atoms,
    "foo" => atom("foo")
);

test_parsing!(lists,
    "(foo bar baz)" => List(vec![atom("foo"), atom("bar"), atom("baz")])
);

test_parsing!(dotted_lists,
    "(foo bar . baz)" => DottedList(vec![atom("foo"), atom("bar")], box atom("baz"))
);

test_parsing!(numbers,
    "13" => Number(13),
    "-6" => Number(-6)
);

test_parsing!(strings,
    "\"hello world\"" => string("hello world"),
    "\"foo\nbar\"" => string("foo\nbar")
);

test_parsing!(booleans,
    "#t" => Boolean(true),
    "#true" => Boolean(true),
    "#f" => Boolean(false),
    "#false" => Boolean(false)
);
