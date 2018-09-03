extern crate parsecute;

use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::response::*;
use parsecute::parsers::monadic::*;

#[test]
fn it_parse_any_macro_seq() {
    let r = seq!((any()));

    assert_eq!('a', r.execute(&"abc".as_bytes(), 0).fold(
        |a, _, _| a as char,
        |_, _| panic!("Parse error"),
    ));
}
/*
#[test]
fn it_parse_any_then_any_macro_seq() {
    let r = seq!((any()) ~ (any()));

    assert_eq!(('a','b'), r.execute(&"abc".as_bytes(), 0).fold(
        |a, _, _| a as char,
        |_, _| panic!("Parse error"),
    ));
}
*/