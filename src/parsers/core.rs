use parsers::response::*;

pub trait Executable<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A>;
}
