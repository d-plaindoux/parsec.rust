extern crate parsecute;

use parsecute::parsers::basic::*;
use parsecute::parsers::core::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::response::*;

#[test]
fn it_parse_with_fmap_success() {
    let r = fmap!(|a:u32| a.to_string(), returns(1));

    assert_eq!("1", r.do_parse(&"a", 0).fold(
        |a, _, _| a,
        |_, _|panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_fmap_reject() {
    let r = fmap!(|a: u32| a.to_string(), fails());

    assert_eq!("0", r.do_parse(&"a", 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, _|"0",
    ));
}

#[test]
fn it_parse_with_bind_success() {
    let r = bind!(|a:u32| Box::new(returns(a + 1)), returns(1));

    assert_eq!(2, r.do_parse(&"a", 0).fold(
        |a, _, _| a,
        |_, _|panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_bind_reject() {
    let r = bind!(|_|Box::new(fails()), returns(1));

    assert_eq!(0, r.do_parse(&"a", 0).fold(
        |_: u32, _, _| panic!("Parse error"),
        |_, _|0,
    ));
}
