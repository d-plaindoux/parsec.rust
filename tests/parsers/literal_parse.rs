extern crate parsecute;

use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::literal::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::response::*;

#[test]
fn it_parse_with_char() {
    let r = 'a';

    assert_eq!(
        (),
        r.parse_only(b"ab", 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_with_char_consumed() {
    let r = 'a';

    assert_eq!(
        true,
        r.parse_only(b"ab", 0)
            .fold(|_, _, b| b, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_with_char_rejected() {
    let r = 'a';

    assert_eq!(
        false,
        r.parse_only(b"b", 0)
            .fold(|_, _, _| panic!("Parse error"), |_, b| b,)
    );
}

#[test]
fn it_parse_with_string() {
    let s = "a".repeat(1024 * 1024);
    let r = s.to_string();

    assert_eq!(
        (),
        r.parse_only(&s.as_bytes(), 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_with_string_consumed() {
    let s = "a".repeat(1024 * 1024);
    let r = s.to_string();

    assert_eq!(
        true,
        r.parse_only(&s.as_bytes(), 0)
            .fold(|_, _, b| b, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_with_string_rejected() {
    let r = "ab".to_string();

    assert_eq!(
        false,
        r.parse_only(b"aa", 0)
            .fold(|_, _, _| panic!("Parse error"), |_, b| b,)
    );
}

#[test]
fn it_parse_with_digit() {
    assert_eq!(
        (),
        digit()
            .parse_only(b"0", 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_with_float() {
    assert_eq!(
        (),
        float()
            .parse_only(b"-1024.32", 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_with_delimited_string() {
    assert_eq!(
        (),
        delimited_string()
            .parse_only(b"\"1024\"", 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_with_delimited_char() {
    assert_eq!(
        'a',
        delimited_char()
            .execute(b"'a'", 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_extracting_float() {
    let p = "Hello<"
        .to_string()
        .then(float())
        .then('>')
        .fmap(|((_, b), _)| b);

    assert_eq!(
        (),
        p.parse_only(b"Hello<42>", 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}

#[test]
fn it_parse_extracting_csv_items() {
    let atom = || take_while(|c| *c != b',');
    let line = atom().then(','.then(atom()).fmap(|(_, b)| b).optrep());

    assert_eq!(
        (),
        line.parse_only(b"a,b,c,d", 0)
            .fold(|a, _, _| a, |_, _| panic!("Parse error"),)
    );
}
