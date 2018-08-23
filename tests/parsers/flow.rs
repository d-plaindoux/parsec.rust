extern crate parsecute;

use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::response::*;

#[test]
fn it_parse_with_seq() {
    let r = any().then(any());

    assert_eq!(('a', 'b'), r.execute(&"ab", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_or_success() {
    let r = returns(2).or(fail());

    assert_eq!(2, r.execute(&"a", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_or_reject() {
    let r = fail().or(returns(2));

    assert_eq!(2, r.execute(&"a", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_opt_success() {
    let r = any().opt();

    assert_eq!(Some('a'), r.execute(&"a", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_opt_success_empty() {
    let r = opt(any());

    assert_eq!(None, r.execute(&"", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_optrep_success() {
    let r = optrep(any());

    let s = 1024 * 1024;
    assert_eq!(s, r.execute(&"a".repeat(s), 0).fold(
        |a, _, _| a.len(),
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_optrep_success_empty() {
    let r = any().optrep();

    assert_eq!(0, r.execute(&"", 0).fold(
        |a, _, _| a.len(),
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_rep_success() {
    let r = any().rep();

    let s = 1024 * 1024;

    assert_eq!(s, r.execute(&"a".repeat(s), 0).fold(
        |a, _, _| a.len(),
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_rep_reject_empty() {
    let r = rep(any());

    assert_eq!(false, r.execute(&"", 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_take_while() {
    let r = take_while(Box::new(|a| *a != 'b'));

    assert_eq!(true, r.execute(&"aaaab", 0).fold(
        |r: Vec<char>, _, _| r.len() == 4,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_take_while_empty() {
    let r = take_while(Box::new(|a| *a != 'b'));

    assert_eq!(true, r.execute(&"b", 0).fold(
        |r: Vec<char>, _, _| r.len() == 0,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_take_while_consumed() {
    let r = take_while(Box::new(|a| *a != 'b'));

    assert_eq!(true, r.execute(&"aaaab", 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}
