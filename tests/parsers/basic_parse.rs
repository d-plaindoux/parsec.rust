use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::response::*;
use parsecute::parsers::flow::skip;

#[test]
fn it_parse_with_returns() {
    let r = returns(1);

    assert_eq!((), r.parse_only(&"a".as_bytes(), 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_returns_no_consumed() {
    let r = returns(1);

    assert_eq!(false, r.parse_only(&"a".as_bytes(), 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}
/*
#[test]
fn it_parse_with_fails_no_consumed() {
    let r = fail();

    assert_eq!(false, r.parse_only(&"a".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}
*/
#[test]
fn it_parse_with_any_success() {
    let r = any();

    assert_eq!((), r.parse_only(&"a".as_bytes(), 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_any_reject() {
    let r = any();

    assert_eq!(false, r.parse_only(&"".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_eos_success() {
    let r = eos();

    assert_eq!((), r.parse_only(&"".as_bytes(), 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_eos_reject() {
    let r = eos();

    assert_eq!(false, r.parse_only(&"a".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_try_any_reject() {
    let r = do_try(any());

    assert_eq!(false, r.parse_only(&"".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_try_any_success() {
    let r = do_try(any());

    assert_eq!(true, r.parse_only(&"a".as_bytes(), 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_satisfy_any_reject() {
    let r = satisfy(any(), |c| *c as char == 'a');

    assert_eq!(true, r.parse_only(&"b".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_satisfy_any_success_unwrap() {
    let r = satisfy(any(), |c| *c as char == 'a');

    assert_eq!(true, r.parse_only(&"a".as_bytes(), 0).fold(
        |_, s, _| s == 1,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_satisfy_any_success() {
    let r = satisfy(any(), |c| *c as char == 'a');

    assert_eq!(true, r.parse_only(&"a".as_bytes(), 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_lazy_any_success() {
    let r = lazy(|| any());

    assert_eq!(true, r.parse_only(&"a".as_bytes(), 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_lazy_any_reject() {
    let r = lazy(|| any());

    assert_eq!(false, r.parse_only(&"".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_skip_success() {
    let r = skip(" \n\r\t".to_string());

    assert_eq!((2, false), r.parse_only(&" \n".as_bytes(), 0).fold(
        |_, o, b| (o, b),
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_noskip_success() {
    let r = skip(" \n\r\t".to_string());

    assert_eq!((0, false), r.parse_only(&"azerty".as_bytes(), 0).fold(
        |_, o, b| (o, b),
        |_, _| panic!("Parse error"),
    ));
}
