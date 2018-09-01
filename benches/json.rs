#[macro_use]
extern crate bencher;
#[macro_use]
extern crate parsecute;

use bencher::{Bencher, black_box};
use parsecute::parsers::basic::*;
use parsecute::parsers::core::*;
use parsecute::parsers::data::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::literal::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::parser::*;
use parsecute::parsers::response::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue<'a> {
    Null(),
    Str(&'a str),
    Num(f64),
    Boolean(bool),
    Array(Vec<JsonValue<'a>>),
    Object(HashMap<&'a str, JsonValue<'a>>),
}

fn json_parser<'a>() -> Parsec<'a, JsonValue<'a>> {
    #[inline]
    fn spaces<E, A>(p: E) -> FMap<And<Skip, (), E, A>, ((), A), A> where E: Parser<A> {
        skip(" \n\r\t".to_string()).then_right(p)
    }

    fn to_str(s: StringLiteral) -> &str {
        let StringLiteral(s, o, n) = s;
        std::str::from_utf8(&s[o..n]).unwrap()
    }

    #[inline]
    fn object<'a>() -> Parsec<'a, JsonValue<'a>> {
        let attribute = || spaces(delimited_string()).then_left(spaces(':')).then(json::<'a>());
        let attributes = attribute().then(spaces(',').then_right(attribute()).optrep()).opt();
        let parser = '{'.then_right(attributes).then_left(spaces('}')).fmap(Box::new(|v| {
            let mut r = HashMap::default();
            if let Some(((k, e), v)) = v {
                r.insert(to_str(k), e);
                for (k, e) in v {
                    r.insert(to_str(k), e);
                }
            }
            JsonValue::Object(r)
        }));

        parsec!('a, parser)
    }

    #[inline]
    fn array<'a>() -> Parsec<'a, JsonValue<'a>> {
        let elements = json::<'a>().then(spaces(',').then_right(json::<'a>()).optrep()).opt();
        let parser = '['.then_right(elements).then_left(spaces(']')).fmap(Box::new(|v| {
            if let Some((e, v)) = v {
                let mut r = v;
                r.insert(0, e);
                JsonValue::Array(r)
            } else {
                JsonValue::Array(Vec::default())
            }
        }));

        parsec!('a, parser)
    }

    #[inline]
    fn json<'a>() -> Parsec<'a, JsonValue<'a>> {
        let parser = lazy!(
            // This trigger should be done automatically in the next version hiding this ugly parse type impersonation
            spaces(lookahead(any()).bind(Box::new(|c| {
                match c as char {
                    '{' => object::<'a>(),
                    '[' => array::<'a>(),
                    '"' => parsec!('a, delimited_string().fmap(Box::new(|v| JsonValue::Str(to_str(v))))),
                    'f' => parsec!('a, "false".fmap(Box::new(|_| JsonValue::Boolean(false)))),
                    't' => parsec!('a, "true".fmap(Box::new(|_| JsonValue::Boolean(true)))),
                    'n' => parsec!('a, "null".fmap(Box::new(|_| JsonValue::Null()))),
                    _   => parsec!('a, float().fmap(Box::new(|v| JsonValue::Num(v.to_native_value())))),
                }
            })))
        );

        parsec!('a, parser)
    }

    parsec!('a,json::<'a>().then_left(spaces(eos())))
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

fn json_canada_pest(b: &mut Bencher) {
    let data = include_bytes!("data/canada_pest.json");
    b.bytes = data.len() as u64;
    parse(json_parser(), b, data)
}

// -------------------------------------------------------------------------------------------------

fn json_canada_nom(b: &mut Bencher) {
    let data = include_bytes!("data/canada_nom.json");
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
            Response { v: Some(_), o: _, c: _ } => (),
            Response { v: None, o, c: _ } => panic!("unable parse stream at character {}", o),
        }
    });
}

benchmark_group!(benches,
json_basic, json_data, json_canada_pest, json_canada_nom, json_apache
);
benchmark_main!(benches);
