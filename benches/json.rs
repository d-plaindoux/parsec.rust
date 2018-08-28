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
        let parser = lazy(Box::new(||
            spaces(constants().or(atoms()).or(object()).or(record()).fmap(Box::new(|_| ())))
        ));

        parsec!(parser)
    }

    parsec!(json().then_left(spaces(eos())))
}

// -------------------------------------------------------------------------------------------------
// Bench mark tests
// -------------------------------------------------------------------------------------------------

fn basic(b: &mut Bencher) {
    let data = b"  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { \"hello\" : \"world\"
  }
  }  ";

    b.bytes = data.len() as u64;
    parse(b, &data[..])
}

fn data(b: &mut Bencher) {
    let data = include_bytes!("data/data.json");
    b.bytes = data.len() as u64;
    parse(b, data)
}

fn canada(b: &mut Bencher) {
    let data = include_bytes!("data/canada.json");
    b.bytes = data.len() as u64;
    parse(b, data)
}

fn apache(b: &mut Bencher) {
    let data = include_bytes!("data/apache_builds.json");
    b.bytes = data.len() as u64;
    parse(b, data)
}

fn parse(b: &mut Bencher, buffer: &[u8]) {
    b.iter(|| {
        let buffer = black_box(buffer);

        match json_parser().execute(buffer, 0) {
            Response::Success(_, _, _) => (),
            Response::Reject(o, _) => panic!("Ouch at {}", o),
        }
    });
}

benchmark_group!(benches, basic, data, canada, apache);
benchmark_main!(benches);

// -------------------------------------------------------------------------------------------------
