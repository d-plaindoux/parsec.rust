use parsers::response::*;

pub trait Parser<A> {}

pub trait Executable<A> where Self: Parser<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A>;
}
