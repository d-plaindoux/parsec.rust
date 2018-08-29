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

impl<'a, E, A, B> Executable<'a, B> for FMap<E, A, B>
    where E: Executable<'a, A> + Parser<A>
{
    fn execute(&self, s: &'a [u8], o: usize) -> Response<B> {
        let FMap(p, f) = self;

        match p.execute(s, o) {
            Response(Some(v), o, b) => response(Some(f(v)), o, b),
            Response(None, o, b) => response(None, o, b)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, E, A, R, B> Executable<'a, B> for Bind<E, A, R, B>
    where E: Executable<'a, A> + Parser<A>,
          R: Executable<'a, B> + Parser<B>
{
    fn execute(&self, s: &'a [u8], o: usize) -> Response<B> {
        let Bind(p, f, _) = self;

        match p.execute(s, o) {
            Response(Some(a1), i1, b1) => {
                match f(a1).execute(s, i1) {
                    Response(Some(a2), i2, b2) => response(Some(a2), i2, b1 || b2),
                    Response(None, i2, b2) => response(None, i2, b1 || b2),
                }
            }
            Response(None, i1, b1) => response(None, i1, b1)
        }
    }
}

// -------------------------------------------------------------------------------------------------
