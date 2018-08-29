#[macro_use]
extern crate bencher;
extern crate parsecute;

use bencher::{Bencher, black_box};
use parsecute::parsers::basic::*;
use parsecute::parsers::core::*;
use parsecute::parsers::data::SubString;
use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::literal::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::parser::*;
use parsecute::parsers::response::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
    Null(),
    Str(&'a str),
    Num(f32),
    Boolean(bool),
    Array(Vec<JsonValue<'a>>),
    Object(HashMap<&'a str, JsonValue<'a>>),
}

fn json_parser<'a>() -> Parsec<'a, JsonValue<'a>> {
    #[inline]
    fn spaces<E, A>(p: E) -> FMap<And<Skip, (), E, A>, ((), A), A> where E: Parser<A> {
        skip(" \n\r\t".to_string()).then_right(p)
    }

    fn to_str<'a>(s: SubString<'a>) -> &'a str {
        let SubString(s, o, n) = s;
        std::str::from_utf8(&s[o..n]).unwrap()
    }

    #[inline]
    fn forget<'a, E, A: 'a>(p: E) -> Parsec<'a, ()> where E: Executable<'a, A> + Parser<A> + 'a {
        Parsec::<'a>(Box::new(p.fmap(Box::new(|_| ()))))
    }
    /*
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
    */
    #[inline]
    fn object<'a>() -> Parsec<'a, JsonValue<'a>> {
        let attribute = || spaces(delimited_string()).then_left(spaces(':')).then(json::<'a>());
        let attributes = attribute().then(spaces(',').then_right(attribute()).optrep()).opt();
        let parser = '{'.then_right(attributes).then_left(spaces('}')).fmap(Box::new(|v| {
            let mut r = HashMap::default();
            if let Some(((k,e), v)) = v {
                r.insert(to_str(k),e);
                for (k,e) in v {
                    r.insert(to_str(k),e);
                }
            }
            JsonValue::Object(r)
        }));
        Parsec::<'a>(Box::new(parser))
    }

    #[inline]
    fn array<'a>() -> Parsec<'a, JsonValue<'a>> {
        let elements = json::<'a>().then(spaces(',').then_right(json::<'a>()).optrep()).opt();
        let parser = '['.then_right(elements).then_left(spaces(']')).fmap(Box::new(|v| {
            let mut r = Vec::default();
            if let Some((e, v)) = v {
                r.push(e);
                for e in v {
                    r.push(e);
                }
            }
            JsonValue::Array(r)
        }));
        Parsec::<'a>(Box::new(parser))
    }

    #[inline]
    fn json<'a>() -> Parsec<'a, JsonValue<'a>> {
        let parser = lazy(Box::new(||
            // This trigger should be done automatically ...
            spaces(lookahead(any()).bind(Box::new(|c| {
                match c as char {
                    '{' => object::<'a>(),
                    '[' => array::<'a>(),
                    '"' => Parsec::<'a>(Box::new(delimited_string().fmap(Box::new(|s| JsonValue::Str(to_str(s)))))),
                    'f' => Parsec::<'a>(Box::new("false".fmap(Box::new(|_| JsonValue::Boolean(false))))),
                    't' => Parsec::<'a>(Box::new("true".fmap(Box::new(|_| JsonValue::Boolean(true))))),
                    'n' => Parsec::<'a>(Box::new("null".fmap(Box::new(|_| JsonValue::Null())))),
                    _ => Parsec::<'a>(Box::new(float().fmap(Box::new(|v| JsonValue::Num(v))))),
                }
            })))
        ));

        // spaces(constants::<'a>().or(object::<'a>()).or(record::<'a>().or(atoms::<'a>())).fmap(Box::new(|_| ())))));
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
    let data = include_bytes!("data/canada_old.json");
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
