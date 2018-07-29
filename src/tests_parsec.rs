use parser::*;
use response::*;

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

#[test]
fn it_parse_with_fmap_success() {
    let r = fmap!(|a:u32| a.to_string(), returns(1));

    assert_eq!("1".to_string(), r.parse("a".to_string()).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_fmap_reject() {
    let r = fmap!(|a: u32| a.to_string(), fails());

    assert_eq!("0".to_string(), r.parse("a".to_string()).fold(
        |_, _, _| panic!("Parse error"),
        |_| "0".to_string(),
    ));
}

#[test]
fn it_parse_with_bind_success() {
    let r = bind!(|a:u32| Box::new(returns(a + 1)), returns(1));

    assert_eq!(2, r.parse("a".to_string()).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_bind_reject() {
    let r = bind!(|_| Box::new(fails()), returns(1));

    assert_eq!(0, r.parse("a".to_string()).fold(
        |_: u32, _, _| panic!("Parse error"),
        |_| 0,
    ));
}

#[test]
fn it_parse_with_and() {
    let r = and!(any(), any());

    assert_eq!(('a', 'b'), r.parse("ab".to_string()).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_or_success() {
    let r = or!(returns(2), fails());

    assert_eq!(2, r.parse("a".to_string()).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_or_reject() {
    let r = or!(fails(), returns(2));

    assert_eq!(2, r.parse("a".to_string()).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_opt_success() {
    let r = opt!(any());

    assert_eq!(Some('a'), r.parse("a".to_string()).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_opt_success_empty() {
    let r = opt!(any());

    assert_eq!(None, r.parse("".to_string()).fold(
        |a, _, _| a,
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_optrep_success() {
    let r = optrep!(any());

    let s = 1024 * 64;
    assert_eq!(s, r.parse("a".repeat(s).to_string()).fold(
        |a, _, _| a.len(),
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_optrep_success_empty() {
    let r = optrep!(any());

    assert_eq!(0, r.parse("".to_string()).fold(
        |a, _, _| a.len(),
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_rep_success() {
    let r = rep!(any());

    let s = 1024 * 256;
    assert_eq!(s, r.parse("a".repeat(s).to_string()).fold(
        |a, _, _| a.len(),
        |_| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_rep_reject_empty() {
    let r = rep!(any());

    assert_eq!(false, r.parse("".to_string()).fold(
        |_, _, _| panic!("Parse error"),
        |b| b,
    ));
}
