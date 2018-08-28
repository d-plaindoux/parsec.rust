#[macro_use]
extern crate bencher;
#[macro_use]
extern crate parsecute;

use bencher::{Bencher, black_box};
use parsecute::parsers::basic::*;
use parsecute::parsers::core::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::literal::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::parser::*;
use parsecute::parsers::response::*;

fn json_parser() -> Parsec<()> {
    #[inline]
    fn spaces<E, A>(p: E) -> FMap<And<Skip, (), E, A>, ((), A), A> where E: Parser<A> {
        skip(" \n\r\t".to_string()).then_right(p)
    }

    #[inline]
    fn constants() -> Parsec<()> {
        let parser = "null".to_string().or("true".to_string()).or("false".to_string()).fmap(Box::new(|_| ()));
        parsec!(parser)
    }

    #[inline]
    fn atoms() -> Parsec<()> {
        let parser = delimited_string().fmap(Box::new(|_| ())).or(float().fmap(Box::new(|_| ())));
        parsec!(parser)
    }

    #[inline]
    fn object() -> Parsec<()> {
        let attribute = || spaces(delimited_string()).then(spaces(':')).then(json()).fmap(Box::new(|_| ()));
        let attributes = attribute().then(spaces(',').then(attribute()).optrep()).opt().fmap(Box::new(|_| ()));
        let parser = '{'.then(attributes).then(spaces('}')).fmap(Box::new(|_| ()));
        parsec!(parser)
    }

    #[inline]
    fn record() -> Parsec<()> {
        let elements = json().then(spaces(',').then(json()).optrep()).opt();
        let parser = '['.then(elements).then(spaces(']')).fmap(Box::new(|_| ()));
        parsec!(parser)
    }

    #[inline]
    fn json() -> Parsec<()> {
        let parser = lazy(Box::new(|| spaces(constants().or(atoms()).or(object()).or(record()).fmap(Box::new(|_| ())))));
        parsec!(parser)
    }

    parsec!(json().then_left(spaces(eos())))
}

// -------------------------------------------------------------------------------------------------
// Bench mark tests
// -------------------------------------------------------------------------------------------------

fn basic_any(b: &mut Bencher) {
    let string = "a".repeat(1024 * 1024);
    let data = string.as_bytes();
    b.bytes = data.len() as u64;
    parse(any().rep(), b, data)
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
    parse('a'.fmap(Box::new(|_| 1)).rep(), b, data)
}

// -------------------------------------------------------------------------------------------------
fn json_basic(b: &mut Bencher) {
    let data = b"  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { \"hello\" : \"world\"
  }
  }  ";

    b.bytes = data.len() as u64;
    parse(json_parser(), b, &data[..])
}

fn json_data(b: &mut Bencher) {
    let data = include_bytes!("data/data.json");
    b.bytes = data.len() as u64;
    parse(json_parser(), b, data)
}

fn json_canada(b: &mut Bencher) {
    let data = include_bytes!("data/canada.json");
    b.bytes = data.len() as u64;
    parse(json_parser(), b, data)
}

fn json_apache(b: &mut Bencher) {
    let data = include_bytes!("data/apache_builds.json");
    b.bytes = data.len() as u64;
    parse(json_parser(), b, data)
}

// -------------------------------------------------------------------------------------------------

fn parse<E, A>(p: E, b: &mut Bencher, buffer: &[u8]) where E: Executable<A> {
    b.iter(|| {
        let buffer = black_box(buffer);

        match p.execute(buffer, 0) {
            Response::Success(_, _, _) => (),
            Response::Reject(o, _) => panic!("Ouch at {}", o),
        }
    });
}

benchmark_group!(benches,
                 basic_any, basic_skip, basic_or, basic_and, basic_fmap,
                 json_basic, json_data, json_canada, json_apache
                );
benchmark_main!(benches);
