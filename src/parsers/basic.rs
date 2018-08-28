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

impl<A> Executable<A> for Return<A> where A: Copy {
    #[inline]
    fn execute(&self, _: &[u8], o: usize) -> Response<A> {
        let Return(v) = self;

        Response::Success(v.clone(), o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl<A> Executable<A> for Fail {
    #[inline]
    fn execute(&self, _: &[u8], o: usize) -> Response<A> {
        Response::Reject(o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<u8> for Any {
    #[inline]
    fn execute(&self, s: &[u8], o: usize) -> Response<u8> {
        if o < s.len() {
            return Response::Success(s[o], o + 1, true);
        }

        return Response::Reject(o, false);
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<()> for Eos {
    #[inline]
    fn execute(&self, s: &[u8], o: usize) -> Response<()> {
        if o < s.len() {
            return Response::Reject(o, false);
        }

        Response::Success((), o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Try<E, A> where E: Executable<A> + Parser<A> {
    fn execute(&self, s: &[u8], o: usize) -> Response<A> {
        let Try(p, _) = self;

        match p.execute(s, o) {
            Response::Reject(o, _) => Response::Reject(o, false),
            other => other
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Lookahead<E, A> where E: Executable<A> + Parser<A> {
    fn execute(&self, s: &[u8], o: usize) -> Response<A> {
        let Lookahead(p, _) = self;

        match p.execute(s, o) {
            Response::Success(v, _, b) => Response::Success(v, o, b),
            other => other
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Satisfy<E, A> where E: Executable<A> + Parser<A> {
    fn execute(&self, s: &[u8], o: usize) -> Response<A> {
        let Satisfy(p, c) = self;

        match p.execute(s, o) {
            Response::Success(a, i, b) => {
                if (c)(&a) {
                    Response::Success(a, i, b)
                } else {
                    Response::Reject(i, b)
                }
            }
            r => r,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Lazy<E, A> where E: Executable<A> + Parser<A> {
    #[inline]
    fn execute(&self, s: &[u8], o: usize) -> Response<A> {
        let Lazy(p, _) = self;

        p().execute(s, o)
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<()> for Skip {

    #[inline]
    fn execute(&self, s: &[u8], o: usize) -> Response<()> {
        let Skip(chars) = self;
        let bytes = chars.as_bytes();
        let mut n = o;

        while n < s.len() && bytes.contains(&s[n]) {
            n += 1;
        }

        Response::Success((), n , false)
    }
}

// -------------------------------------------------------------------------------------------------

