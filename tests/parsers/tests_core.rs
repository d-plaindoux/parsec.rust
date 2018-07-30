use parsecute::parsers::basic::*;
use parsecute::parsers::core::*;
use parsecute::parsers::response::*;

#[test]
fn it_parse_with_returns() {
    let r = returns(1);

    assert_eq!(1, r.parse("a".to_string()).fold(
        |a: u32, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_fails() {
    let r = fails();

    assert_eq!(0, r.parse("a".to_string()).fold(
        |_: u32, _, _| panic!("Parse error"),
        |_| 0,
    ));
}

#[test]
fn it_parse_with_any_success() {
    let r = any();

    assert_eq!('a', r.parse("a".to_string()).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_try_any_reject() {
    let r = do_try!(any());

    assert_eq!(false, r.parse("".to_string()).fold(
        |_, _, _| panic!("Parse error"),
        |b| b,
    ));
}

#[test]
fn it_parse_with_try_any_success() {
    let r = do_try!(any());

    assert_eq!(true, r.parse("a".to_string()).fold(
        |_, _, b| b,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_satisfy_any_reject() {
    let r = satisfy!(any(), |c:&char| *c == 'a');

    assert_eq!(true, r.parse("b".to_string()).fold(
        |_, _, _| panic!("Parse error"),
        |b| b,
    ));
}

#[test]
fn it_parse_with_satisfy_any_success() {
    let r = satisfy!(any(), |c:&char| *c == 'a');

    assert_eq!(true, r.parse("a".to_string()).fold(
        |_, _, b| b,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_lookahead_any_reject() {
    let r = lookahead!(any());

    assert_eq!(false, r.parse("".to_string()).fold(
        |_, _, _| panic!("Parse error"),
        |b| b,
    ));
}

#[test]
fn it_parse_with_lookahead_any_success() {
    let r = lookahead!(any());

    assert_eq!(true, r.parse("a".to_string()).fold(
        |_, _, b| b,
        |_| panic!("Parse error"),
    ));
}
