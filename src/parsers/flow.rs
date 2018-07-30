use parsers::core::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Flow
// -------------------------------------------------------------------------------------------------

pub struct And<A, B> { p1: Box<Parser<A>>, p2: Box<Parser<B>> }

impl<A, B> Parser<(A, B)> for And<A, B> {
    fn parse(&self, s: &str, o: usize) -> Response<(A, B)> {
        match self.p1.parse(s, o) {
            Response::Success(a1, i1, b1) => {
                print!("i2 = {} \n",i1);
                match self.p2.parse(s, i1) {
                    Response::Success(a2, i2, b2) => Response::Success((a1, a2), i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2),
                }
            }
            Response::Reject(b1) => Response::Reject(b1)
        }
    }
}

#[inline]
pub fn and<A, B>(p1: Box<Parser<A>>, p2: Box<Parser<B>>) -> And<A, B> {
    And { p1, p2 }
}

#[macro_export]
macro_rules! and {
    ( $p1:expr , $p2:expr ) => {
        and(Box::new($p1), Box::new($p2))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Or<A> { p1: Box<Parser<A>>, p2: Box<Parser<A>> }

impl<A> Parser<A> for Or<A> {
    fn parse(&self, s: &str, o: usize) -> Response<A> {
        match self.p1.parse(s, o) {
            Response::Success(a1, i1, b1) => Response::Success(a1, i1, b1),
            Response::Reject(b1) => {
                match self.p2.parse(s, o) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2)
                }
            }
        }
    }
}

#[inline]
pub fn or<A>(p1: Box<Parser<A>>, p2: Box<Parser<A>>) -> Or<A> {
    Or { p1, p2 }
}

#[macro_export]
macro_rules! or {
    ( $p1:expr , $p2:expr ) => {
        or(Box::new($p1), Box::new($p2))
    };
}

// -------------------------------------------------------------------------------------------------
// Occurrences
// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! opt {
    ( $p:expr ) => {
        or!(fmap!(|a| Some(a), $p), returns(None))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Repeat<A> { opt: bool, p: Box<Parser<A>> }

impl<A> Parser<Vec<A>> for Repeat<A> {
    fn parse(&self, s: &str, o: usize) -> Response<Vec<A>> {
        let mut result: Vec<A> = Vec::with_capacity(13);
        let mut offset = o;
        let mut consumed = false;
        let mut parsed = true;

        while parsed {
            match self.p.parse(s, offset) {
                Response::Success(a1, i1, b1) => {
                    result.push(a1);
                    offset = i1;
                    consumed = consumed || b1;
                }
                _ => {
                    parsed = false;
                }
            }
        }

        if self.opt || result.len() > 0 {
            return Response::Success(result, offset, consumed);
        }

        return Response::Reject(consumed);
    }
}

#[inline]
pub fn optrep<A>(p: Box<Parser<A>>) -> Repeat<A> {
    Repeat { opt: true, p }
}

#[macro_export]
macro_rules! optrep {
    ( $p:expr ) => {
        optrep(Box::new($p))
    };
}

#[inline]
pub fn rep<A>(p: Box<Parser<A>>) -> Repeat<A> {
    Repeat { opt: false, p }
}

#[macro_export]
macro_rules! rep {
    ( $p:expr ) => {
        rep(Box::new($p))
    };
}
