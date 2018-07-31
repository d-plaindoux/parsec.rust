extern crate parsecute;

use parsecute::parsers::chars::*;
use parsecute::parsers::core::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::response::*;


#[test]
fn it_parse_with_char() {
    let r = 'a';

    assert_eq!('a', r.do_parse(&"ab", 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_char_consumed() {
    let r = 'a';

    assert_eq!(true, r.do_parse(&"ab", 0).fold(
        |_, _, b| b,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_char_rejected() {
    let r = 'a';

    assert_eq!(false, r.do_parse(&"b", 0).fold(
        |_, _, _| panic!("Parse error"),
        |b| b,
    ));
}

#[test]
fn it_parse_with_string() {
    let s = "a".repeat(1024 * 1024);
    let r = s.to_string();

    assert_eq!(r, r.do_parse(&s, 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_string_consumed() {
    let s = "a".repeat(1024 * 1024);
    let r = s.to_string();

    assert_eq!(true, r.do_parse(&s, 0).fold(
        |_, _, b| b,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_string_rejected() {
    let r = "ab".to_string();

    assert_eq!(false, r.do_parse(&"aa", 0).fold(
        |_, _, _| panic!("Parse error"),
        |b| b,
    ));
}

#[test]
fn it_parse_with_digit() {
    assert_eq!('0', digit().do_parse(&"0", 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_number() {
    let p = and!(or!('+','-'),fmap!(|a:Vec<char>| a.into_iter().collect(), rep!(digit())));

    assert_eq!(-10, p.do_parse(&"-10", 0).fold(
        |(a, b): (char, String), _, _| (a.to_string() + b.as_str()).parse::<i32>().unwrap(),
        |_| panic!("Parse error"),
    ));
}

