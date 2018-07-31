extern crate parsecute;

use parsecute::parsers::core::*;
use parsecute::parsers::response::*;

#[test]
fn it_parse_with_char() {
    let r = 'a';

    assert_eq!('a', r.parse(&"ab", 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_char_consumed() {
    let r = 'a';

    assert_eq!(true, r.parse(&"ab", 0).fold(
        |_, _, b| b,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_char_rejected() {
    let r = 'a';

    assert_eq!(false, r.parse(&"b", 0).fold(
        |_, _, _| panic!("Parse error"),
        |b| b,
    ));
}

#[test]
fn it_parse_with_string() {
    let s = "a".repeat(1024 * 1024);
    let r = s.to_string();

    assert_eq!(r, r.parse(&s, 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_string_consumed() {
    let s = "a".repeat(1024 * 1024);
    let r = s.to_string();

    assert_eq!(true, r.parse(&s, 0).fold(
        |_, _, b| b,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_string_rejected() {
    let r = "ab".to_string();

    assert_eq!(false, r.parse(&"aa", 0).fold(
        |_, _, _| panic!("Parse error"),
        |b| b,
    ));
}

#[test]
fn it_parse_with_char_predicate() {
    let r : fn(char) -> bool = |a| match a {
        '0'...'9' => true,
        _ => false
    };

    assert_eq!('0', r.parse(&"0", 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

