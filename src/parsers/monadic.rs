use parsers::execution::*;
use parsers::parser::*;
use parsers::response::*;
use std::marker::PhantomData;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub struct FMap<E, A, B>(E, Box<Fn(A) -> B>) where E: Parser<A>;

impl<E, A, B> Parser<B> for FMap<E, A, B> where E: Parser<A> {}

pub trait FMapOperation<E, A, B> where E: Parser<A> {
    fn fmap(self, f: Box<(Fn(A) -> B)>) -> FMap<E, A, B>;
}

impl<E, A, B> FMapOperation<E, A, B> for E where E: Parser<A> {
    #[inline]
    fn fmap(self, f: Box<(Fn(A) -> B)>) -> FMap<E, A, B> {
        FMap(self, f)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Bind<E, A, R, B>(E, Box<Fn(A) -> R>, PhantomData<B>) where E: Parser<A>, R: Parser<B>;

impl<E, A, R, B> Parser<B> for Bind<E, A, R, B> where E: Parser<A>, R: Parser<B> {}

pub trait BindOperation<E, A, R, B> where E: Parser<A>, R: Parser<B> {
    fn bind(self, f: Box<(Fn(A) -> R)>) -> Bind<E, A, R, B>;
}

impl<E, A, R, B> BindOperation<E, A, R, B> for E where E: Parser<A>, R: Parser<B> {
    #[inline]
    fn bind(self, f: Box<(Fn(A) -> R)>) -> Bind<E, A, R, B> {
        Bind(self, f, PhantomData)
    }
}

// -------------------------------------------------------------------------------------------------
// Parser execution
// -------------------------------------------------------------------------------------------------

impl<E, A, B> Executable<B> for FMap<E, A, B>
    where E: Executable<A> + Parser<A>
{
    fn execute(&self, s: &[u8], o: usize) -> Response<B> {
        let FMap(p, f) = self;

        match p.execute(s, o) {
            Response::Reject(o, b) => Response::Reject(o, b),
            Response::Success(v, o, b) => Response::Success(f(v), o, b)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<E, A, R, B> Executable<B> for Bind<E, A, R, B>
    where E: Executable<A> + Parser<A>,
          R: Executable<B> + Parser<B>
{
    fn execute(&self, s: &[u8], o: usize) -> Response<B> {
        let Bind(p, f, _) = self;

        match p.execute(s, o) {
            Response::Success(a1, i1, b1) => {
                match f(a1).execute(s, i1) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(i2, b2) => Response::Reject(i2, b1 || b2),
                }
            }
            Response::Reject(i1, b1) => Response::Reject(i1, b1)
        }
    }
}

// -------------------------------------------------------------------------------------------------
