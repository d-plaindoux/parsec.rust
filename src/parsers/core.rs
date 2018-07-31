use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub trait Parser<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<A>;
}

pub type Parsec<A> = Box<Parser<A>>;
