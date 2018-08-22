use core::marker::PhantomData;
use parsers::core::Executable;
use parsers::core::Parser;
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

pub fn fail() -> Fail {
    return Fail();
}

// -------------------------------------------------------------------------------------------------

pub struct Any();

impl Parser<char> for Any {}

pub fn any() -> Any {
    return Any();
}

// -------------------------------------------------------------------------------------------------

pub struct Eos();

impl Parser<()> for Eos {}

pub fn eos() -> Eos {
    return Eos();
}

// -------------------------------------------------------------------------------------------------

pub struct Try<E, A>(pub E, pub PhantomData<A>) where E: Parser<A>;

impl<E, A> Parser<A> for Try<E, A> where E: Parser<A> {}

pub fn do_try<E, A>(p: E) -> Try<E, A> where E: Parser<A> {
    Try(p, PhantomData)
}

// -------------------------------------------------------------------------------------------------

pub struct Lookahead<E, A>(pub E, pub PhantomData<A>) where E: Parser<A>;

impl<E, A> Parser<A> for Lookahead<E, A> where E: Parser<A> {}

pub fn lookahead<E, A>(p: E) -> Lookahead<E, A> where E: Parser<A> {
    Lookahead(p, PhantomData)
}

// -------------------------------------------------------------------------------------------------

pub struct Satisfy<E, A>(pub E, pub Box<Fn(&A) -> bool>) where E: Parser<A>;

impl<E, A> Parser<A> for Satisfy<E, A> where E: Parser<A> {}

pub fn satisfy<E, A>(p: E, f: Box<Fn(&A) -> bool>) -> Satisfy<E, A> where E: Parser<A> {
    Satisfy(p, f)
}

// -------------------------------------------------------------------------------------------------
// Parser execution
// -------------------------------------------------------------------------------------------------

impl<A> Executable<A> for Return<A> where A: Copy {
    fn execute(&self, _: &str, o: usize) -> Response<A> {
        let Return(v) = self;

        Response::Success(v.clone(), o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl<A> Executable<A> for Fail {
    fn execute(&self, _: &str, o: usize) -> Response<A> {
        Response::Reject(o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<char> for Any {
    fn execute(&self, s: &str, o: usize) -> Response<char> {
        if o >= s.len() {
            return Response::Reject(o, false);
        }

        Response::Success(s[o..(o + 1)].chars().next().unwrap(), o + 1, true)
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<()> for Eos {
    fn execute(&self, s: &str, o: usize) -> Response<()> {
        if o < s.len() {
            return Response::Reject(o, false);
        }

        Response::Success((), o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Try<E, A> where E: Executable<A> + Parser<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A> {
        let Try(p, _) = self;

        match p.execute(s, o) {
            Response::Reject(o, _) => Response::Reject(o, false),
            other => other
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Lookahead<E, A> where E: Executable<A> + Parser<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A> {
        let Lookahead(p, _) = self;

        match p.execute(s, o) {
            Response::Success(v, _, b) => Response::Success(v, o, b),
            other => other
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Satisfy<E, A> where E: Executable<A> + Parser<A> {
    fn execute(&self, s: &str, o: usize) -> Response<A> {
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
