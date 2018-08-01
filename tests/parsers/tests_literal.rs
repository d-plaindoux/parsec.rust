extern crate parsecute;

use parsecute::parsers::core::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::literal::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::response::*;


#[test]
fn it_parse_with_char() {
    let r = 'a';

    assert_eq!('a', r.do_parse(&"ab", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_char_consumed() {
    let r = 'a';

    assert_eq!(true, r.do_parse(&"ab", 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_char_rejected() {
    let r = 'a';

    assert_eq!(false, r.do_parse(&"b", 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_string() {
    let s = "a".repeat(1024 * 1024);
    let r = s.to_string();

    assert_eq!(r, r.do_parse(&s, 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_string_consumed() {
    let s = "a".repeat(1024 * 1024);
    let r = s.to_string();

    assert_eq!(true, r.do_parse(&s, 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_string_rejected() {
    let r = "ab".to_string();

    assert_eq!(false, r.do_parse(&"aa", 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_digit() {
    assert_eq!('0', digit().do_parse(&"0", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_natural() {
    assert_eq!(-1024, natural().do_parse(&"-1024", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_delimited_string() {
    assert_eq!("1024", string_delim().do_parse(&"\"1024\"", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_delimited_char() {
    assert_eq!('a', char_delim().do_parse(&"'a'", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_extracting_natural() {
    let p = fmap!(|(_,(b,_))| b, and!("Hello<".to_string(), natural(), '>'));

    assert_eq!(42, p.do_parse(&"Hello<42>", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error")
    ));
}
