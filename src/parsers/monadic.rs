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
    fn fmap_box(self, f: Box<Fn(A) -> B>) -> FMap<E, A, B>;
    fn fmap<F>(self, f: F) -> FMap<E, A, B> where F: (Fn(A) -> B) + 'static;
    fn map<F>(self, f: F) -> FMap<E, A, B> where F: (Fn(A) -> B) + 'static;
}

impl<E, A, B> FMapOperation<E, A, B> for E where E: Parser<A> {
    #[inline]
    fn fmap_box(self, f: Box<Fn(A) -> B>) -> FMap<E, A, B> {
        FMap(self, f)
    }
    #[inline]
    fn fmap<F>(self, f: F) -> FMap<E, A, B> where F: (Fn(A) -> B) + 'static {
        FMap(self, Box::new(f))
    }
    #[inline]
    fn map<F>(self, f: F) -> FMap<E, A, B> where F: (Fn(A) -> B) + 'static {
        FMap(self, Box::new(f))
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Bind<E, A, R, B>(E, Box<Fn(A) -> R>, PhantomData<B>) where E: Parser<A>, R: Parser<B>;

impl<E, A, R, B> Parser<B> for Bind<E, A, R, B> where E: Parser<A>, R: Parser<B> {}

pub trait BindOperation<E, A, R, B> where E: Parser<A>, R: Parser<B> {
    fn bind_box(self, f: Box<Fn(A) -> R>) -> Bind<E, A, R, B>;
    fn bind<F>(self, f: F) -> Bind<E, A, R, B> where F: (Fn(A) -> R) + 'static;
    fn flat_map<F>(self, f: F) -> Bind<E, A, R, B> where F: (Fn(A) -> R) + 'static;
}

impl<E, A, R, B> BindOperation<E, A, R, B> for E where E: Parser<A>, R: Parser<B> {
    #[inline]
    fn bind_box(self, f: Box<Fn(A) -> R>) -> Bind<E, A, R, B> {
        Bind(self, f, PhantomData)
    }
    #[inline]
    fn bind<F>(self, f: F) -> Bind<E, A, R, B> where F: (Fn(A) -> R) + 'static {
        Bind(self, Box::new(f), PhantomData)
    }
    #[inline]
    fn flat_map<F>(self, f: F) -> Bind<E, A, R, B> where F: (Fn(A) -> R) + 'static {
        self.bind(f)
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
