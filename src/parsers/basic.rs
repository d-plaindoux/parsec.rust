use parsers::core::Parser;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Core
// -------------------------------------------------------------------------------------------------

pub struct Returns<A> { a: A }

impl<A> Parser<A> for Returns<A> where A: Copy {
    fn parse(&self, _: &str, o: usize) -> Response<A> {
        Response::Success(self.a, o, false)
    }
}

#[inline]
pub fn returns<A>(a: A) -> Returns<A> {
    return Returns { a };
}

// -------------------------------------------------------------------------------------------------

pub struct Fails;

impl<A> Parser<A> for Fails {
    fn parse(&self, _: &str, _: usize) -> Response<A> {
        return Response::Reject(false);
    }
}

#[inline]
pub fn fails() -> Fails {
    return Fails {};
}

// -------------------------------------------------------------------------------------------------

pub struct Any;

impl Parser<char> for Any {
    fn parse(&self, s: &str, o: usize) -> Response<char> {
        if o  >= s.len() {
            return Response::Reject(false);
        }

        return Response::Success(s[o..(o+1)].chars().next().unwrap(), o + 1, true);
    }
}

#[inline]
pub fn any() -> Any {
    return Any {};
}

// -------------------------------------------------------------------------------------------------

pub struct Try<A> { p: Box<Parser<A>> }

impl<A> Parser<A> for Try<A> {
    fn parse(&self, s: &str, o: usize) -> Response<A> {
        match self.p.parse(s, o) {
            Response::Reject(_) => Response::Reject(false),
            r => r
        }
    }
}

#[inline]
pub fn try<A>(p: Box<Parser<A>>) -> Try<A> {
    Try { p }
}

#[macro_export]
macro_rules! do_try {
    ( $x:expr ) => {
        try(Box::new($x))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Satisfy<A> { p: Box<Parser<A>>, c: Box<Fn(&A) -> bool> }

impl<A> Parser<A> for Satisfy<A> {
    fn parse(&self, s: &str, o: usize) -> Response<A> {
        match self.p.parse(s, o) {
            Response::Success(a, i, b) => {
                if (self.c)(&a) {
                    Response::Success(a, i, b)
                } else {
                    Response::Reject(b)
                }
            }
            r => r,
        }
    }
}

#[inline]
pub fn satisfy<A>(p: Box<Parser<A>>, c: Box<Fn(&A) -> bool>) -> Satisfy<A> {
    Satisfy { p, c }
}

#[macro_export]
macro_rules! satisfy {
    ( $p:expr, $c:expr ) => {
        satisfy(Box::new($p), Box::new($c))
    };
}
// -------------------------------------------------------------------------------------------------

pub struct Lookahead<A> { p: Box<Parser<A>> }

impl<A> Parser<A> for Lookahead<A> {
    fn parse(&self, s: &str, o: usize) -> Response<A> {
        match self.p.parse(s, o) {
            Response::Success(a, _, b) => Response::Success(a, o, b),
            _ => Response::Reject(false),
        }
    }
}

#[inline]
pub fn lookahead<A>(p: Box<Parser<A>>) -> Lookahead<A> {
    Lookahead { p }
}

#[macro_export]
macro_rules! lookahead {
    ( $x:expr ) => {
        lookahead(Box::new($x))
    };
}
