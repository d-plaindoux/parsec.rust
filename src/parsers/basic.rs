use core::marker::PhantomData;
use parsers::execution::*;
use parsers::parser::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub struct Return<E>(pub E);

impl<E> Parser<E> for Return<E> {}

pub fn returns<A>(v: A) -> Return<A> {
    Return(v)
}

// -------------------------------------------------------------------------------------------------

pub struct Fail();

impl<E> Parser<E> for Fail {}

#[inline]
pub fn fail() -> Fail {
    return Fail();
}

// -------------------------------------------------------------------------------------------------

pub struct Any();

impl Parser<u8> for Any {}

#[inline]
pub fn any() -> Any {
    return Any();
}

// -------------------------------------------------------------------------------------------------

pub struct Eos();

impl Parser<()> for Eos {}

#[inline]
pub fn eos() -> Eos {
    return Eos();
}

// -------------------------------------------------------------------------------------------------

pub struct Try<E, A>(pub E, pub PhantomData<A>) where E: Parser<A>;

impl<E, A> Parser<A> for Try<E, A> where E: Parser<A> {}

#[inline]
pub fn do_try<E, A>(p: E) -> Try<E, A> where E: Parser<A> {
    Try(p, PhantomData)
}

// -------------------------------------------------------------------------------------------------

pub struct Lookahead<E, A>(pub E, pub PhantomData<A>) where E: Parser<A>;

impl<E, A> Parser<A> for Lookahead<E, A> where E: Parser<A> {}

#[inline]
pub fn lookahead<E, A>(p: E) -> Lookahead<E, A> where E: Parser<A> {
    Lookahead(p, PhantomData)
}

// -------------------------------------------------------------------------------------------------

pub struct Satisfy<E, A>(pub E, pub Box<Fn(&A) -> bool>) where E: Parser<A>;

impl<E, A> Parser<A> for Satisfy<E, A> where E: Parser<A> {}

#[inline]
pub fn satisfy<E, A>(p: E, f: Box<Fn(&A) -> bool>) -> Satisfy<E, A> where E: Parser<A> {
    Satisfy(p, f)
}

pub trait SatisfyOperation<E, A> where E: Parser<A> {
    #[inline]
    fn satisfy(self, f: Box<Fn(&A) -> bool>) -> Satisfy<E, A>;
}

impl<E, A> SatisfyOperation<E, A> for E where E: Parser<A> {
    #[inline]
    fn satisfy(self, f: Box<(Fn(&A) -> bool)>) -> Satisfy<E, A> {
        satisfy(self, f)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Lazy<E, A>(pub Box<Fn() -> E>, pub PhantomData<A>) where E: Parser<A>;

impl<E, A> Parser<A> for Lazy<E, A> where E: Parser<A> {}

#[inline]
pub fn lazy<E, A>(p: Box<Fn() -> E>) -> Lazy<E, A> where E: Parser<A> {
    Lazy(p, PhantomData)
}

// -------------------------------------------------------------------------------------------------

pub struct Skip(pub String);

impl Parser<()> for Skip {}

#[inline]
pub fn skip(s: String) -> Skip {
    Skip(s)
}

// -------------------------------------------------------------------------------------------------
// Parser execution
// -------------------------------------------------------------------------------------------------

impl<'a, A> Executable<'a, A> for Return<A> where A: Copy {
    #[inline]
    fn execute(&self, _: &'a [u8], o: usize) -> Response<A> {
        let Return(v) = self;

        response(Some(v.clone()), o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, A> Executable<'a, A> for Fail {
    #[inline]
    fn execute(&self, _: &'a [u8], o: usize) -> Response<A> {
        response(None, o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, u8> for Any {
    #[inline]
    fn execute(&self, s: &'a [u8], o: usize) -> Response<u8> {
        if o < s.len() {
            return response(Some(s[o]), o + 1, true);
        }

        return response(None, o, false);
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, ()> for Eos {
    #[inline]
    fn execute(&self, s: &'a [u8], o: usize) -> Response<()> {
        if o < s.len() {
            return response(None, o, false);
        }

        response(Some(()), o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, A, E> Executable<'a, A> for Try<E, A> where E: Executable<'a, A> + Parser<A> {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<A> {
        let Try(p, _) = self;

        match p.execute(s, o) {
            Response(None, o, _) => Response(None, o, false),
            other => other
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, A, E> Executable<'a, A> for Lookahead<E, A> where E: Executable<'a, A> + Parser<A> {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<A> {
        let Lookahead(p, _) = self;

        match p.execute(s, o) {
            Response(Some(v), _, b) => response(Some(v), o, b),
            other => other
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, A, E> Executable<'a, A> for Satisfy<E, A> where E: Executable<'a, A> + Parser<A> {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<A> {
        let Satisfy(p, c) = self;

        match p.execute(s, o) {
            Response(Some(a), i, b) => {
                if (c)(&a) {
                    response(Some(a), i, b)
                } else {
                    response(None, i, b)
                }
            }
            r => r,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, A, E> Executable<'a, A> for Lazy<E, A> where E: Executable<'a, A> + Parser<A> {
    #[inline]
    fn execute(&self, s: &'a [u8], o: usize) -> Response<A> {
        let Lazy(p, _) = self;

        p().execute(s, o)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, ()> for Skip {
    #[inline]
    fn execute(&self, s: &'a [u8], o: usize) -> Response<()> {
        let Skip(chars) = self;
        let bytes = chars.as_bytes();
        let mut n = o;

        while n < s.len() && bytes.contains(&s[n]) {
            n += 1;
        }

        Response(Some(()), n, false)
    }
}

// -------------------------------------------------------------------------------------------------

