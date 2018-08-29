#[macro_use]
extern crate bencher;
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

fn json_parser<'a>() -> Parsec<'a, ()> {
    #[inline]
    fn spaces<E, A>(p: E) -> FMap<And<Skip, (), E, A>, ((), A), A> where E: Parser<A> {
        skip(" \n\r\t".to_string()).then_right(p)
    }

    #[inline]
    fn constants<'a>() -> Parsec<'a, ()> {
        let parser = "null".or("true").or("false").fmap(Box::new(|_| ()));
        Parsec::<'a>(Box::new(parser))
    }

    #[inline]
    fn atoms<'a>() -> Parsec<'a, ()> {
        let parser = delimited_string().fmap(Box::new(|_| ())).or(float().fmap(Box::new(|_| ())));
        Parsec::<'a>(Box::new(parser))
    }

    #[inline]
    fn object<'a>() -> Parsec<'a, ()> {
        let attribute = || spaces(delimited_string()).then(spaces(':')).then(json::<'a>()).fmap(Box::new(|_| ()));
        let attributes = attribute().then(spaces(',').then(attribute()).optrep()).opt().fmap(Box::new(|_| ()));
        let parser = '{'.then(attributes).then(spaces('}')).fmap(Box::new(|_| ()));
        Parsec::<'a>(Box::new(parser))
    }

    #[inline]
    fn record<'a>() -> Parsec<'a, ()> {
        let elements = json::<'a>().then(spaces(',').then(json::<'a>()).optrep()).opt();
        let parser = '['.then(elements).then(spaces(']')).fmap(Box::new(|_| ()));
        Parsec::<'a>(Box::new(parser))
    }

    #[inline]
    fn json<'a>() -> Parsec<'a, ()> {
        let parser = lazy(Box::new(|| spaces(constants::<'a>().or(object::<'a>()).or(record::<'a>().or(atoms::<'a>())).fmap(Box::new(|_| ())))));
        Parsec::<'a>(Box::new(parser))
    }

    Parsec::<'a>(Box::new(json::<'a>().then_left(spaces(eos()))))
}

// -------------------------------------------------------------------------------------------------
// JSon benchmarks
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

// -------------------------------------------------------------------------------------------------

fn json_data(b: &mut Bencher) {
    let data = include_bytes!("data/data.json");
    b.bytes = data.len() as u64;
    parse(json_parser(), b, data)
}

// -------------------------------------------------------------------------------------------------

fn json_canada(b: &mut Bencher) {
    let data = include_bytes!("data/canada.json");
    b.bytes = data.len() as u64;
    parse(json_parser(), b, data)
}

// -------------------------------------------------------------------------------------------------

fn json_apache(b: &mut Bencher) {
    let data = include_bytes!("data/apache_builds.json");
    b.bytes = data.len() as u64;
    parse(json_parser(), b, data)
}

// -------------------------------------------------------------------------------------------------
// Main parse function used for benchmarking
// -------------------------------------------------------------------------------------------------

fn parse<'a, E, A>(p: E, b: &mut Bencher, buffer: &'a [u8]) where E: Executable<'a, A> {
    b.iter(|| {
        let buffer = black_box(buffer);

        match p.execute(buffer, 0) {
            Response(Some(_), _, _) => (),
            Response(None, o, _) => panic!("unable parse stream at character {}", o),
        }
    });
}

benchmark_group!(benches,
                 json_basic, json_data, json_canada, json_apache
                );
benchmark_main!(benches);
