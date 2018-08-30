use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::response::*;

#[test]
fn it_execute_with_returns() {
    let r = returns(1);

    assert_eq!(1, r.execute(&"a".as_bytes(), 0).fold(
        |a: u32, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_only_with_returns() {
    let r = returns(1);

    assert_eq!((), r.parse_only(&"a".as_bytes(), 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_returns_no_consumed() {
    let r = returns(1);

    assert_eq!(false, r.execute(&"a".as_bytes(), 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_fails_no_consumed() {
    let r = fail();

    assert_eq!(false, r.execute(&"a".as_bytes(), 0).fold(
        |_: u32, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_execute_with_any_success() {
    let r = any();

    assert_eq!('a', r.execute(&"a".as_bytes(), 0).fold(
        |a, _, _| a as char,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_any_reject() {
    let r = any();

    assert_eq!(false, r.execute(&"".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_execute_with_eos_success() {
    let r = eos();

    assert_eq!((), r.execute(&"".as_bytes(), 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_eos_reject() {
    let r = eos();

    assert_eq!(false, r.execute(&"a".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_execute_with_try_any_reject() {
    let r = do_try(any());

    assert_eq!(false, r.execute(&"".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_execute_with_try_any_success() {
    let r = do_try(any());

    assert_eq!(true, r.execute(&"a".as_bytes(), 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_satisfy_any_reject() {
    let r = satisfy(any(), Box::new(|c| *c as char == 'a'));

    assert_eq!(true, r.execute(&"b".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_execute_with_satisfy_any_success_unwrap() {
    let r = satisfy(any(), Box::new(|c| *c as char == 'a'));

    assert_eq!(true, r.execute(&"a".as_bytes(), 0).fold(
        |_, s, _| s == 1,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_satisfy_any_success() {
    let r = satisfy(any(), Box::new(|c| *c as char == 'a'));

    assert_eq!(true, r.execute(&"a".as_bytes(), 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_lookahead_any_reject() {
    let r = lookahead(any());

    assert_eq!(false, r.execute(&"".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_execute_with_lookahead_any_success() {
    let r = lookahead(any());

    assert_eq!(true, r.execute(&"a".as_bytes(), 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_lookahead_any_success_no_unwrap() {
    let r = lookahead(any());

    assert_eq!(true, r.execute(&"a".as_bytes(), 0).fold(
        |_, s, _| s == 0,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_lazy_any_success() {
    let r = lazy(Box::new(|| any()));

    assert_eq!(true, r.execute(&"a".as_bytes(), 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_lazy_any_reject() {
    let r = lazy(Box::new(|| any()));

    assert_eq!(false, r.execute(&"".as_bytes(), 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_execute_with_skip_success() {
    let r = skip(" \n\r\t".to_string());

    assert_eq!((2, false), r.execute(&" \n".as_bytes(), 0).fold(
        |_, o, b| (o, b),
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_execute_with_noskip_success() {
    let r = skip(" \n\r\t".to_string());

    assert_eq!((0, false), r.execute(&"azerty".as_bytes(), 0).fold(
        |_, o, b| (o, b),
        |_, _| panic!("Parse error"),
    ));
}
