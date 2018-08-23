// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

use parsers::execution::Executable;

pub trait Parser<A> {}

// -------------------------------------------------------------------------------------------------
// Basic Parser used for type simplification
// -------------------------------------------------------------------------------------------------

pub struct Parsec<A>(pub Box<Executable<A>>);

impl<A> Parser<A> for Parsec<A> {}

pub fn parsec<A>(p: Box<Executable<A>>) -> Parsec<A> {
    Parsec(p)
}