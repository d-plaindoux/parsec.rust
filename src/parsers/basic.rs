use parsers::core::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Core
// -------------------------------------------------------------------------------------------------

pub struct Returns<A> { a: A }

impl<A> ParserTrait<A> for Returns<A> where A: Copy {
    fn do_parse(&self, _: &str, o: usize) -> Response<A> {
        Response::Success(self.a, o, false)
    }
}

#[inline]
pub fn returns<A>(a: A) -> Returns<A> {
    return Returns { a };
}

// -------------------------------------------------------------------------------------------------

pub struct Fails;

impl<A> ParserTrait<A> for Fails {
    fn do_parse(&self, _: &str, _: usize) -> Response<A> {
        return Response::Reject(false);
    }
}

#[inline]
pub fn fails() -> Fails {
    return Fails {};
}

// -------------------------------------------------------------------------------------------------

pub struct Eos;

impl ParserTrait<()> for Eos {
    fn do_parse(&self, s: &str, o: usize) -> Response<()> {
        if o < s.len() {
            return Response::Reject(false);
        }

        return Response::Success((), o, true);
    }
}

#[inline]
pub fn eos() -> Eos {
    return Eos {};
}

// -------------------------------------------------------------------------------------------------

pub struct Any;

impl ParserTrait<char> for Any {
    fn do_parse(&self, s: &str, o: usize) -> Response<char> {
        if o >= s.len() {
            return Response::Reject(false);
        }

        return Response::Success(s[o..(o + 1)].chars().next().unwrap(), o + 1, true);
    }
}

#[inline]
pub fn any() -> Any {
    return Any {};
}

// -------------------------------------------------------------------------------------------------

pub struct Try<A> { p: Parsec<A> }

impl<A> ParserTrait<A> for Try<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<A> {
        match self.p.do_parse(s, o) {
            Response::Reject(_) => Response::Reject(false),
            r => r
        }
    }
}

#[inline]
pub fn try<A>(p: Parsec<A>) -> Try<A> {
    Try { p }
}

#[macro_export]
macro_rules! do_try {
    ( $x:expr ) => {
        try(Box::new($x))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Satisfy<A> { p: Parsec<A>, c: Box<Fn(&A) -> bool> }

impl<A> ParserTrait<A> for Satisfy<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<A> {
        match self.p.do_parse(s, o) {
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
pub fn satisfy<A>(p: Parsec<A>, c: Box<Fn(&A) -> bool>) -> Satisfy<A> {
    Satisfy { p, c }
}

#[macro_export]
macro_rules! satisfy {
    ( $p:expr, $c:expr ) => {
        satisfy(Box::new($p), Box::new($c))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Lookahead<A> { p: Parsec<A> }

impl<A> ParserTrait<A> for Lookahead<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<A> {
        match self.p.do_parse(s, o) {
            Response::Success(a, _, b) => Response::Success(a, o, b),
            _ => Response::Reject(false),
        }
    }
}

#[inline]
pub fn lookahead<A>(p: Parsec<A>) -> Lookahead<A> {
    Lookahead { p }
}

#[macro_export]
macro_rules! lookahead {
    ( $x:expr ) => {
        lookahead(Box::new($x))
    };
}
