use parsers::execution::*;
use parsers::parser::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Basic Parser used for type simplification
// -------------------------------------------------------------------------------------------------

pub struct Parsec<'a, A>(pub Box<Executable<'a, A> + 'a>);

impl<'a, A> Parser<A> for Parsec<'a, A> {}

pub fn parsec<'a, A>(p: Box<Executable<'a, A> + 'a>) -> Parsec<'a, A> {
    Parsec::<'a>(p)
}

#[macro_export]
macro_rules! parsec {
    ($a:lifetime, $e:expr) => { Parsec::<$a>(Box::new($e)) };
    ($e:expr) => { parsec(Box::new($e)) };
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
