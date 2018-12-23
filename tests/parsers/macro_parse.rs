extern crate parsecute;

use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::response::*;

#[test]
fn it_parse_any_macro_seq() {
    let r = seq!((any()));

    assert_eq!(
        'a',
        r.execute(b"abc", 0)
            .fold(|a, _, _| a as char, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_any_then_any_macro_seq() {
    let r = foreach!(
        ('\'')
        b <- (any())
        if (b as char != '\'')
        ('\'')
        yield (b)
    );

    assert_eq!(
        'b',
        r.execute(b"'b'", 0)
            .fold(|b, _, _| b as char, |_, _| panic!("Parse error"),)
    );
}
