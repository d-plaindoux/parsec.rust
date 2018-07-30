use parsers::core::Parser;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Core
// -------------------------------------------------------------------------------------------------

pub struct Returns<A> { a: A }

impl<A> Parser<A> for Returns<A> where A: Copy {
    fn parse(&self, s: String) -> Response<A> {
        Response::Success(self.a, s, false)
    }
}

pub fn returns<A>(a: A) -> Returns<A> {
    return Returns { a };
}

// -------------------------------------------------------------------------------------------------

pub struct Fails;

impl<A> Parser<A> for Fails {
    fn parse(&self, _: String) -> Response<A> {
        return Response::Reject(false);
    }
}

pub fn fails() -> Fails {
    return Fails {};
}

// -------------------------------------------------------------------------------------------------

pub struct Any;

impl Parser<char> for Any {
    fn parse(&self, s: String) -> Response<char> {
        if s.len() < 1 {
            return Response::Reject(false);
        }

        return Response::Success(s.chars().next().unwrap(), s[1..].to_string(), true);
    }
}

pub fn any() -> Any {
    return Any {};
}

// -------------------------------------------------------------------------------------------------

pub struct Try<A> { p: Box<Parser<A>> }

impl<A> Parser<A> for Try<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p.parse(s) {
            Response::Reject(_) => Response::Reject(false),
            r => r
        }
    }
}

pub fn try<A>(p: Box<Parser<A>>) -> Try<A> {
    Try { p }
}

macro_rules! do_try {
    ( $x:expr ) => {
        try(Box::new($x))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Satisfy<A> { p: Box<Parser<A>>, c: Box<Fn(&A) -> bool> }

impl<A> Parser<A> for Satisfy<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p.parse(s) {
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

pub fn satisfy<A>(p: Box<Parser<A>>, c: Box<Fn(&A) -> bool>) -> Satisfy<A> {
    Satisfy { p, c }
}

macro_rules! satisfy {
    ( $p:expr, $c:expr ) => {
        satisfy(Box::new($p), Box::new($c))
    };
}
// -------------------------------------------------------------------------------------------------

pub struct Lookahead<A> { p: Box<Parser<A>> }

impl<A> Parser<A> for Lookahead<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p.parse(s.clone()) {
            Response::Success(a, _, b) => Response::Success(a, s, b),
            _ => Response::Reject(false),
        }
    }
}

pub fn lookahead<A>(p: Box<Parser<A>>) -> Lookahead<A> {
    Lookahead { p }
}

macro_rules! lookahead {
    ( $x:expr ) => {
        lookahead(Box::new($x))
    };
}
