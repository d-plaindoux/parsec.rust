#[macro_use]
extern crate bencher;
extern crate parsecute;

use bencher::{Bencher, black_box};
use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::literal::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::response::*;
use parsecute::parsers::parser::Parser;

// -------------------------------------------------------------------------------------------------
// Basic benchmarks
// -------------------------------------------------------------------------------------------------

fn basic_any(b: &mut Bencher) {
    let string = "a".repeat(1024 * 1024);
    let data = string.as_bytes();
    b.bytes = data.len() as u64;
    parse(any().rep(), b, data)
}

// -------------------------------------------------------------------------------------------------

fn basic_do_try(b: &mut Bencher) {
    let string = "a".repeat(1024 * 1024);
    let data = string.as_bytes();
    b.bytes = data.len() as u64;
    parse(do_try(any()).rep(), b, data)
}

// -------------------------------------------------------------------------------------------------

fn basic_skip(b: &mut Bencher) {
    let string = " \t\n\r".repeat(1024 * 512);
    let data = string.as_bytes();
    b.bytes = data.len() as u64;
    parse(skip(" \t\n\r".to_string()), b, data)
}

// -------------------------------------------------------------------------------------------------

fn basic_or(b: &mut Bencher) {
    let string = "ab".repeat(1024 * 512);
    let data = string.as_bytes();
    b.bytes = data.len() as u64;
    parse('a'.or('b').rep(), b, data)
}

// -------------------------------------------------------------------------------------------------

fn basic_and(b: &mut Bencher) {
    let string = "ab".repeat(1024 * 512);
    let data = string.as_bytes();
    b.bytes = data.len() as u64;
    parse('a'.then('b').rep(), b, data)
}

// -------------------------------------------------------------------------------------------------

fn basic_fmap(b: &mut Bencher) {
    let string = "a".repeat(1024 * 1024);
    let data = string.as_bytes();
    b.bytes = data.len() as u64;
    parse('a'.fmap(|_| 1).rep(), b, data)
}

// -------------------------------------------------------------------------------------------------
// Literal benchmarks
// -------------------------------------------------------------------------------------------------

fn literal_delimited_string(b: &mut Bencher) {
    let string = "\"te\\\"st\"".repeat(1024);
    let data = string.as_bytes();
    b.bytes = data.len() as u64;
    parse(delimited_string().rep(), b, data)
}

// -------------------------------------------------------------------------------------------------

fn literal_float(b: &mut Bencher) {
    let string = "-12.34000056".repeat(1024);
    let data: &[u8] = string.as_bytes();
    b.bytes = data.len() as u64;
    parse(float().rep(), b, data)
}

// -------------------------------------------------------------------------------------------------
// Main parse function used for benchmarking
// -------------------------------------------------------------------------------------------------

fn parse<'a, E, A>(p: E, b: &mut Bencher, buffer: &'a [u8]) where E: Executable<'a, A> + Parser<A> {
    let r = p.then(eos());

    b.iter(|| {
        let buffer = black_box(buffer);

        match r.execute(buffer, 0) {
            Response { v: Some(_), o: _, c: _ } => (),
            Response { v: None, o, c: _ } => panic!("unable parse stream at character {}", o),
        }
    });
}

benchmark_group!(benches,
                 basic_any, basic_skip, basic_do_try, basic_or, basic_and, basic_fmap,
                 literal_delimited_string, literal_float
                );
benchmark_main!(benches);
