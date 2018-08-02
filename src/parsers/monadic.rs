use parsers::core::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Monadic
// -------------------------------------------------------------------------------------------------

pub struct FMap<A, B> { f: Box<Fn(A) -> B>, p: Parsec<A> } // Can we remove this Box

impl<A, B> ParserTrait<B> for FMap<A, B> {
    fn do_parse(&self, s: &str, o: usize) -> Response<B> {
        match self.p.do_parse(s, o) {
            Response::Success(a, i, b) => Response::Success((self.f)(a), i, b),
            Response::Reject(i, b) => Response::Reject(i, b)
        }
    }
}

#[inline]
pub fn fmap<A, B>(f: Box<Fn(A) -> B>, p: Parsec<A>) -> FMap<A, B> {
    FMap { f, p }
}

#[macro_export]
macro_rules! fmap {
    ( $f:expr , $($p:expr),+ ) => { fmap(Box::new($f), Box::new(seq!($($p),+))) };
}

// -------------------------------------------------------------------------------------------------

pub struct Bind<A, B> { f: Box<Fn(A) -> Parsec<B>>, p: Parsec<A> } // Can we remove this Box

impl<A, B> ParserTrait<B> for Bind<A, B> {
    fn do_parse(&self, s: &str, o: usize) -> Response<B> {
        match self.p.do_parse(s, o) {
            Response::Success(a1, i1, b1) => {
                match (self.f)(a1).do_parse(s, i1) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(i2, b2) => Response::Reject(i2, b1 || b2),
                }
            }
            Response::Reject(i1, b1) => Response::Reject(i1, b1)
        }
    }
}

#[inline]
pub fn bind<A, B>(f: Box<Fn(A) -> Parsec<B>>, p: Parsec<A>) -> Bind<A, B> {
    Bind { f, p }
}

#[macro_export]
macro_rules! bind {
    ( $f:expr , $($p:expr),+ ) => { bind(Box::new($f), Box::new(seq!($($p),+))) };
}
