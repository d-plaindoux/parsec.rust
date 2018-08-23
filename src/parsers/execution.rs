use parsers::parser::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Executable type definition
// -------------------------------------------------------------------------------------------------

pub trait Executable<A> where Self: Parser<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A>;
}
