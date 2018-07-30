use parsers::core::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Monadic
// -------------------------------------------------------------------------------------------------

pub struct Join<A> { p: Box<Parser<Box<Parser<A>>>> } // How this Box of Box can be simplified ?

impl<A> Parser<A> for Join<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p.parse(s) {
            Response::Success(a1, i1, b1) => {
                match a1.parse(i1.to_string()) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2),
                }
            }
            Response::Reject(b1) => Response::Reject(b1)
        }
    }
}

pub fn join<A>(p: Box<Parser<Box<Parser<A>>>>) -> Join<A> {
    Join { p }
}

macro_rules! join {
    ( $x:expr ) => {
        join(Box::new($x))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct FMap<A, B> { f: Box<Fn(A) -> B>, p: Box<Parser<A>> } // Can we remove this Box

impl<A, B> Parser<B> for FMap<A, B> {
    fn parse(&self, s: String) -> Response<B> {
        match self.p.parse(s) {
            Response::Success(a, i, b) => Response::Success((self.f)(a), i, b),
            Response::Reject(b) => Response::Reject(b)
        }
    }
}

pub fn fmap<A, B>(f: Box<Fn(A) -> B>, p: Box<Parser<A>>) -> FMap<A, B> {
    FMap { f, p }
}

macro_rules! fmap {
    ( $f:expr , $x:expr ) => {
        fmap(Box::new($f), Box::new($x))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Bind<A, B> { f: Box<Fn(A) -> Box<Parser<B>>>, p: Box<Parser<A>> } // Can we remove this Box

impl<A, B> Parser<B> for Bind<A, B> {
    fn parse(&self, s: String) -> Response<B> {
        match self.p.parse(s) {
            Response::Reject(b1) => Response::Reject(b1),
            Response::Success(a1, i1, b1) => {
                match (self.f)(a1).parse(i1.to_string()) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2),
                }
            }
        }
    }
}

pub fn bind<A, B>(f: Box<Fn(A) -> Box<Parser<B>>>, p: Box<Parser<A>>) -> Bind<A, B> {
    Bind { f, p }
}

macro_rules! bind {
    ( $f:expr , $p:expr ) => {
        bind(Box::new($f), Box::new($p))
    };
}