use parsers::response::*;
use std::marker::PhantomData;

pub trait Parser<A> {}

pub trait Executable<A> where Self: Parser<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A>;
}

pub struct Parsec<A>(Box<Executable<A>>);

impl<A> Parser<A> for Parsec<A> {}

impl<A> Executable<A> for Parsec<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A> {
        let Parsec(e) = self;

        e.execute(s, o)
    }
}

pub fn parsec<A>(p: Box<Executable<A>>) -> Parsec<A> {
    Parsec(p)
}