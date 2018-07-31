use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub trait ParserTrait<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<A>;
}

pub type Parsec<A> = Box<ParserTrait<A>>;

pub struct Parser<A> { p: Parsec<A> }

impl<A> ParserTrait<A> for Parser<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<A> {
        self.p.do_parse(s, o)
    }
}

#[inline]
pub fn parser<A>(p: Parsec<A>) -> Parser<A> {
    Parser { p }
}

#[macro_export]
macro_rules! parser {
    ($e:expr) => { parser(Box::new($e)) };
}