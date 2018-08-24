use parsers::execution::*;
use parsers::parser::*;
use parsers::response::Response;

// -------------------------------------------------------------------------------------------------
// Basic Parser used for type simplification
// -------------------------------------------------------------------------------------------------

pub struct Parsec<A>(pub Box<Executable<A>>);

impl<A> Parser<A> for Parsec<A> {}

pub fn parsec<A>(p: Box<Executable<A>>) -> Parsec<A> {
    Parsec(p)
}

// -------------------------------------------------------------------------------------------------
// Basic Parser executable implementation
// -------------------------------------------------------------------------------------------------

impl<A> Executable<A> for Parsec<A> {
    fn execute(&self, s: &[u8], o: usize) -> Response<A> {
        let Parsec(e) = self;

        e.execute(s, o)
    }
}
