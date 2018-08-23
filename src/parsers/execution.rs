use parsers::response::*;
use parsers::parser::*;

// -------------------------------------------------------------------------------------------------
// Executable type definition
// -------------------------------------------------------------------------------------------------

pub trait Executable<A> where Self: Parser<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A>;
}

// -------------------------------------------------------------------------------------------------
// Basic Parser executable implementation
// -------------------------------------------------------------------------------------------------

impl<A> Executable<A> for Parsec<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A> {
        let Parsec(e) = self;

        e.execute(s, o)
    }
}