use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub trait Parser<A> {
    fn parse(&self, s: &str, o: usize) -> Response<A>;
}

// -------------------------------------------------------------------------------------------------
// Reification
// -------------------------------------------------------------------------------------------------

pub struct Parsec<A> { p: Box<Parser<A>> }

impl<A> Parser<A> for Parsec<A> {
    fn parse(&self, s: &str, o: usize) -> Response<A> {
        self.p.parse(s, o)
    }
}

#[macro_export]
macro_rules! parser {
    ( $x:expr ) => {
        Parsec { p : Box::new($x) }
    };
}

