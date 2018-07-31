extern crate parsecute;

use parsecute::parsers::basic::*;
use parsecute::parsers::core::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::response::*;

#[test]
fn it_parse_with_and() {
    let r = and!(any(), any());

    assert_eq!(('a', 'b'), r.do_parse(&"ab", 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_or_success() {
    let r = or!(returns(2),fails());

    assert_eq!(2, r.do_parse(&"a", 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_or_reject() {
    let r = or!(fails(), returns(2));

    assert_eq!(2, r.do_parse(&"a", 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_opt_success() {
    let r = opt!(any());

    assert_eq!(Some('a'), r.do_parse(&"a", 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_opt_success_empty() {
    let r = opt!(any());

    assert_eq!(None, r.do_parse(&"", 0).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_optrep_success() {
    let r = optrep!(any());

    let s = 1024 * 1024;
    assert_eq!(s, r.do_parse(&"a".repeat(s), 0).fold(
        |a, _, _| a.len(),
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_optrep_success_empty() {
    let r = optrep!(any());

    assert_eq!(0, r.do_parse(&"", 0).fold(
        |a, _, _| a.len(),
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_rep_success() {
    let r = rep!(any());

    let s = 1024 * 1024;

    assert_eq!(s, r.do_parse(&"a".repeat(s), 0).fold(
        |a, _, _| a.len(),
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_rep_reject_empty() {
    let r = rep!(any());

    assert_eq!(false, r.do_parse(&"", 0).fold(
        |_, _, _| panic!("Parse error"),
        |b| b,
    ));
}

#[test]
fn it_parse_with_take_while() {
    let r = take_while!(|a| *a != 'b');

    assert_eq!(true, r.do_parse(&"aaaab", 0).fold(
        |r: Vec<char>, _, _| r.len() == 4,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_take_while_empty() {
    let r = take_while!(|a| *a != 'b');

    assert_eq!(true, r.do_parse(&"b", 0).fold(
        |r: Vec<char>, _, _| r.len() == 0,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_take_while_consumed() {
    let r = take_while!(|a| *a != 'b');

    assert_eq!(true, r.do_parse(&"aaaab", 0).fold(
        |_, _, b| b,
        |_| panic!("Parse error"),
    ));
}