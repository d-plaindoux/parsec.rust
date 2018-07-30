use parsers::basic::*;
use parsers::core::*;
use parsers::flow::*;
use parsers::monadic::*;
use parsers::response::*;

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
