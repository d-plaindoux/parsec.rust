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
    fn fmap<F: 'static>(self, f: F) -> FMap<E, A, B> where F: Fn(A) -> B;
    fn flat_map<F: 'static>(self, f: F) -> FMap<E, A, B> where F: Fn(A) -> B;
}

impl<E, A, B> FMapOperation<E, A, B> for E where E: Parser<A> {
    #[inline]
    fn fmap<F: 'static>(self, f: F) -> FMap<E, A, B> where F: Fn(A) -> B {
        FMap(self, Box::new(f))
    }
    #[inline]
    fn flat_map<F: 'static>(self, f: F) -> FMap<E, A, B> where F: Fn(A) -> B {
        FMap(self, Box::new(f))
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Bind<E, A, R, B>(E, Box<Fn(A) -> R>, PhantomData<B>) where E: Parser<A>, R: Parser<B>;

impl<E, A, R, B> Parser<B> for Bind<E, A, R, B> where E: Parser<A>, R: Parser<B> {}

pub trait BindOperation<E, A, R, B> where E: Parser<A>, R: Parser<B> {
    fn bind<F: 'static>(self, f: F) -> Bind<E, A, R, B> where F: (Fn(A) -> R);
}

impl<E, A, R, B> BindOperation<E, A, R, B> for E where E: Parser<A>, R: Parser<B> {
    #[inline]
    fn bind<F: 'static>(self, f: F) -> Bind<E, A, R, B> where F: (Fn(A) -> R) {
        Bind(self, Box::new(f), PhantomData)
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
        let r = p.execute(s, o);

        match r.v {
            Some(v) => response(Some(f(v)), r.o, r.c),
            _ => response(None, r.o, r.c)
        }
    }
}

impl<'a, E, A, B> Parsable<'a, B> for FMap<E, A, B>
    where E: Parsable<'a, A> + Parser<A>
{
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let FMap(p, _) = self;
        let r = p.parse_only(s, o);

        match r.v {
            Some(_) => response(Some(()), r.o, r.c),
            _ => response(None, r.o, r.c)
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
        let r1 = p.execute(s, o);

        match r1.v {
            Some(a1) => {
                let r2 = f(a1).execute(s, r1.o);

                match r2.v {
                    Some(a2) => response(Some(a2), r2.o, r1.c || r2.c),
                    _ => response(None, r2.o , r1.c || r2.c),
                }
            }
            _ => response(None, r1.o, r1.c)
        }
    }
}

// -------------------------------------------------------------------------------------------------
