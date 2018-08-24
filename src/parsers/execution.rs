use parsers::parser::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Executable type definition
// -------------------------------------------------------------------------------------------------

pub trait Executable<A> where Self: Parser<A> {
    fn execute(&self, s: &[u8], o: usize) -> Response<A>;
}
