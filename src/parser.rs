use response::*;
use std::collections::LinkedList;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub trait Parser<A> {
    fn parse(&self, s: String) -> Response<A>;
}

// -------------------------------------------------------------------------------------------------
// Reification
// -------------------------------------------------------------------------------------------------

#[allow(dead_code)]
pub struct Parsec<A> { p: Box<Parser<A>> }

impl<A> Parser<A> for Parsec<A> {
    fn parse(&self, s: String) -> Response<A> {
        self.p.parse(s)
    }
}

#[macro_use]
#[allow(unused_macros)]
macro_rules! parser {
    ( $x:expr ) => {
        Parsec { p : Box::new($x) }
    };
}

// -------------------------------------------------------------------------------------------------
// Core
// -------------------------------------------------------------------------------------------------

pub struct Returns<A> { a: A }

impl<A> Parser<A> for Returns<A> where A: Copy {
    fn parse(&self, s: String) -> Response<A> {
        Response::Success(self.a, s, false)
    }
}

#[allow(dead_code)]
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

#[macro_use]
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

#[macro_use]
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

#[macro_use]
macro_rules! lookahead {
    ( $x:expr ) => {
        lookahead(Box::new($x))
    };
}

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

#[allow(dead_code)]
pub fn join<A>(p: Box<Parser<Box<Parser<A>>>>) -> Join<A> {
    Join { p }
}

#[macro_use]
#[allow(unused_macros)]
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

#[macro_use]
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

#[macro_use]
macro_rules! bind {
    ( $f:expr , $p:expr ) => {
        bind(Box::new($f), Box::new($p))
    };
}

// -------------------------------------------------------------------------------------------------
// Flow
// -------------------------------------------------------------------------------------------------

pub struct And<A, B> { p1: Box<Parser<A>>, p2: Box<Parser<B>> }

impl<A, B> Parser<(A, B)> for And<A, B> {
    fn parse(&self, s: String) -> Response<(A, B)> {
        match self.p1.parse(s) {
            Response::Success(a1, i1, b1) => {
                match self.p2.parse(i1.to_string()) {
                    Response::Success(a2, i2, b2) => Response::Success((a1, a2), i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2),
                }
            }
            Response::Reject(b1) => Response::Reject(b1)
        }
    }
}

pub fn and<A, B>(p1: Box<Parser<A>>, p2: Box<Parser<B>>) -> And<A, B> {
    And { p1, p2 }
}

#[macro_use]
macro_rules! and {
    ( $p1:expr , $p2:expr ) => {
        and(Box::new($p1), Box::new($p2))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Or<A> { p1: Box<Parser<A>>, p2: Box<Parser<A>> }

impl<A> Parser<A> for Or<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p1.parse(s.clone()) { // Borrowing ...
            Response::Success(a1, i1, b1) => Response::Success(a1, i1, b1),
            Response::Reject(b1) => {
                match self.p2.parse(s) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2)
                }
            }
        }
    }
}

pub fn or<A>(p1: Box<Parser<A>>, p2: Box<Parser<A>>) -> Or<A> {
    Or { p1, p2 }
}

#[macro_use]
macro_rules! or {
    ( $p1:expr , $p2:expr ) => {
        or(Box::new($p1), Box::new($p2))
    };
}

// -------------------------------------------------------------------------------------------------
// Occurrences
// -------------------------------------------------------------------------------------------------

#[macro_use]
macro_rules! opt {
    ( $p:expr ) => {
        or!(fmap!(|a| Some(a), $p), returns(None))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Repeat<A> { opt: bool, p: Box<Parser<A>> }

impl<A> Parser<LinkedList<A>> for Repeat<A> {
    fn parse(&self, s: String) -> Response<LinkedList<A>> {
        let mut result: LinkedList<A> = Default::default();
        let mut input = s;
        let mut consumed = false;
        let mut parsed = true;

        while parsed {
            match self.p.parse(input.clone()) {
                Response::Success(a1, i1, b1) => {
                    result.push_back(a1);
                    input = i1;
                    consumed = consumed || b1;
                }
                _ => {
                    parsed = false;
                }
            }
        }

        if self.opt || result.len() > 0 {
            return Response::Success(result, input, consumed);
        }

        return Response::Reject(consumed);
    }
}

pub fn optrep<A>(p: Box<Parser<A>>) -> Repeat<A> {
    Repeat { opt: true, p }
}

#[macro_use]
macro_rules! optrep {
    ( $p:expr ) => {
        optrep(Box::new($p))
    };
}

pub fn rep<A>(p: Box<Parser<A>>) -> Repeat<A> {
    Repeat { opt: false, p }
}


macro_rules! rep {
    ( $p:expr ) => {
        rep(Box::new($p))
    };
}

// -------------------------------------------------------------------------------------------------
// Char/String
// -------------------------------------------------------------------------------------------------

// TODO

// -------------------------------------------------------------------------------------------------
