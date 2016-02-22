#![feature(box_syntax)]

extern crate r7rs;

mod shared;

use shared::*;
use r7rs::expression;
use r7rs::LispValue::*;

macro_rules! test_parsing {
    ($name:ident, $($inp:expr => $out:expr),+) => (
        #[test]
        fn $name() {
            $(assert_eq!(expression(&$inp), Ok($out));)+
        }
    );
}

test_parsing!(atoms,
    "foo" => atom("foo")
);

test_parsing!(quoting,
    "'foo" => List(vec![atom("quote"), atom("foo")]),
    "(foo 'bar)" => List(vec![atom("foo"), List(vec![atom("quote"), atom("bar")])])
);

test_parsing!(lists,
    "(foo bar baz)" => List(vec![atom("foo"), atom("bar"), atom("baz")])
);

test_parsing!(dotted_lists,
    "(foo bar . baz)" => DottedList(vec![atom("foo"), atom("bar")], box atom("baz"))
);

test_parsing!(numbers,
    "13" => number(13),
    "-6" => number(-6),
    "4.0" => number(4),
    "-.0" => number(0),
    "-9/3" => number(-3),
    "+320/4" => number(80),
    "+6.0" => number(6)
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
