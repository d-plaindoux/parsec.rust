extern crate parsecute;

use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::response::*;

#[test]
fn it_parse_with_fmap_success() {
    let r = returns(1).fmap(|a: u32| a.to_string());

    assert_eq!(
        "1",
        r.execute(b"a", 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_with_fmap_reject() {
    let r = fail().fmap(|a: u32| a.to_string());

    assert_eq!(
        "0",
        r.execute(b"a", 0)
            .fold(|_, _, _| panic!("Parse error"), |_, _| "0",)
    );
}

#[test]
fn it_parse_with_bind_success() {
    let r = returns(1).bind(|a: u32| returns(a + 1));

    assert_eq!(
        2,
        r.execute(b"a", 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_with_bind_reject() {
    let r = returns(1).bind(|_: u32| fail());

    assert_eq!(
        0,
        r.execute(b"a", 0)
            .fold(|_: u32, _, _| panic!("Parse error"), |_, _| 0,)
    );
}
