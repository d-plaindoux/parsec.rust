use parsers::execution::*;
use parsers::parser::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Basic Parser used for type simplification
// -------------------------------------------------------------------------------------------------

pub struct Parsec<'a, A>(pub Box<dyn Executable<'a, A> + 'a>);

impl<'a, A> Parser<A> for Parsec<'a, A> {}

pub fn parsec<'a, A>(p: Box<dyn Executable<'a, A> + 'a>) -> Parsec<'a, A> {
    Parsec::<'a>(p)
}

// -------------------------------------------------------------------------------------------------
// Basic Parser executable implementation
// -------------------------------------------------------------------------------------------------

impl<'a, A> Executable<'a, A> for Parsec<'a, A> {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<A> {
        let Parsec(e) = self;

        e.execute(s, o)
    }
}
