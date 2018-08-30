use core::marker::PhantomData;
use parsers::basic::*;
use parsers::execution::*;
use parsers::monadic::*;
use parsers::parser::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub struct Or<E, R, A>(pub E, pub R, PhantomData<A>) where E: Parser<A>, R: Parser<A>;

impl<E, R, A> Parser<A> for Or<E, R, A> where E: Parser<A>, R: Parser<A> {}

pub trait OrOperation<E, R, A> where E: Parser<A>, R: Parser<A> {
    fn or(self, R) -> Or<E, R, A>;
}

impl<E, R, A> OrOperation<E, R, A> for E where E: Parser<A>, R: Parser<A> {
    #[inline]
    fn or(self, a: R) -> Or<E, R, A> {
        Or(self, a, PhantomData)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct And<E, A, R, B>(pub E, pub R, PhantomData<A>, PhantomData<B>) where E: Parser<A>, R: Parser<B>;

impl<E, A, R, B> Parser<(A, B)> for And<E, A, R, B> where E: Parser<A>, R: Parser<B> {}

pub trait AndOperation<E, A, R, B> where E: Parser<A>, R: Parser<B> {
    fn then(self, R) -> And<E, A, R, B>;
    fn then_left(self, b: R) -> FMap<And<E, A, R, B>, (A, B), A>;
    fn then_right(self, b: R) -> FMap<And<E, A, R, B>, (A, B), B>;
}

impl<E, A, R, B> AndOperation<E, A, R, B> for E where E: Parser<A>, R: Parser<B> {
    #[inline]
    fn then(self, b: R) -> And<E, A, R, B> {
        And(self, b, PhantomData, PhantomData)
    }
    #[inline]
    fn then_left(self, b: R) -> FMap<And<E, A, R, B>, (A, B), A> {
        And(self, b, PhantomData, PhantomData).fmap(Box::new(|(a, _)| a))
    }
    #[inline]
    fn then_right(self, b: R) -> FMap<And<E, A, R, B>, (A, B), B> {
        And(self, b, PhantomData, PhantomData).fmap(Box::new(|(_, b)| b))
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Opt<E, A>(E, PhantomData<A>) where E: Parser<A>;

impl<E, A> Parser<Option<A>> for Opt<E, A> where E: Parser<A> {}

#[inline]
pub fn opt<E, A>(p: E) -> Opt<E, A> where E: Parser<A> {
    Opt(p, PhantomData)
}

//  -------------------------------------------------------------------------------------------------

pub struct Repeat<E, A>(bool, E, PhantomData<A>) where E: Parser<A>;

impl<E, A> Parser<Vec<A>> for Repeat<E, A> where E: Parser<A> {}

#[inline]
pub fn optrep<E, A>(p: E) -> Repeat<E, A> where E: Parser<A> {
    Repeat(true, p, PhantomData)
}

#[inline]
pub fn rep<E, A>(p: E) -> Repeat<E, A> where E: Parser<A> {
    Repeat(false, p, PhantomData)
}

//  -------------------------------------------------------------------------------------------------

pub trait RepeatOperation<E, A> where E: Parser<A> {
    fn opt(self) -> Opt<E, A>;
    fn rep(self) -> Repeat<E, A>;
    fn optrep(self) -> Repeat<E, A>;
}

impl<E, A> RepeatOperation<E, A> for E where E: Parser<A> {
    #[inline]
    fn opt(self) -> Opt<E, A> {
        opt(self)
    }

    #[inline]
    fn rep(self) -> Repeat<E, A> {
        rep(self)
    }

    #[inline]
    fn optrep(self) -> Repeat<E, A> {
        optrep(self)
    }
}

//  -------------------------------------------------------------------------------------------------

pub type TypeWhile = Repeat<Satisfy<Any, u8>, u8>;

#[inline]
pub fn take_while(f: Box<(Fn(&u8) -> bool)>) -> TypeWhile {
    any().satisfy(f).optrep()
}

pub type TakeOne = Try<Satisfy<Any, u8>, u8>;

#[inline]
pub fn take_one(f: Box<(Fn(&u8) -> bool)>) -> TakeOne {
    do_try(any().satisfy(f))
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

impl<'a, E, R, A> Executable<'a, A> for Or<E, R, A>
    where E: Executable<'a, A> + Parser<A>,
          R: Executable<'a, A> + Parser<A>
{
    fn execute(&self, s: &'a [u8], o: usize) -> Response<A> {
        let Or(p1, p2, _) = self;
        let r = p1.execute(s, o);

        match r.v {
            Some(_) => r,
            _ => {
                if r.c {
                    response(None, r.o, r.c)
                } else {
                    p2.execute(s, o)
                }
            }
        }
    }
}

impl<'a, E, R, A> Parsable<'a, A> for Or<E, R, A>
    where E: Parsable<'a, A> + Parser<A>,
          R: Parsable<'a, A> + Parser<A>
{
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let Or(p1, p2, _) = self;
        let r = p1.parse_only(s, o);

        match r.v {
            Some(_) => r,
            _ => {
                if r.c {
                    response(None, r.o, r.c)
                } else {
                    p2.parse_only(s, o)
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, E, A, R, B> Executable<'a, (A, B)> for And<E, A, R, B>
    where E: Executable<'a, A> + Parser<A>,
          R: Executable<'a, B> + Parser<B>
{
    fn execute(&self, s: &'a [u8], o: usize) -> Response<(A, B)> {
        let And(p1, p2, _, _) = self;
        let r1 = p1.execute(s, o);


        match r1.v {
            Some(a1) => {
                let r2 = p2.execute(s, r1.o);

                match r2.v {
                    Some(a2) => response(Some((a1, a2)), r2.o, r1.c || r2.c),
                    _ => response(None, r2.o, r1.c || r2.c),
                }
            }
            _ => response(None, r1.o, r1.c)
        }
    }
}

impl<'a, E, A, R, B> Parsable<'a, (A, B)> for And<E, A, R, B>
    where E: Parsable<'a, A> + Parser<A>,
          R: Parsable<'a, B> + Parser<B>
{
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let And(p1, p2, _, _) = self;
        let r1 = p1.parse_only(s, o);


        match r1.v {
            Some(_) => {
                let r2 = p2.parse_only(s, r1.o);

                match r2.v {
                    Some(_) => response(Some(()), r2.o, r1.c || r2.c),
                    _ => response(None, r2.o, r1.c || r2.c),
                }
            }
            _ => response(None, r1.o, r1.c)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, E, A> Executable<'a, Option<A>> for Opt<E, A>
    where E: Executable<'a, A> + Parser<A>
{
    fn execute(&self, s: &'a [u8], o: usize) -> Response<Option<A>> {
        let Opt(p, _) = self;
        let r = p.execute(s, o);

        match r.v {
            Some(a) => response(Some(Some(a)), r.o, r.c),
            None if r.c == false => response(Some(None), o, r.c),
            None => response(None, o, r.c)
        }
    }
}

impl<'a, E, A> Parsable<'a, Option<A>> for Opt<E, A>
    where E: Parsable<'a, A> + Parser<A>
{
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let Opt(p, _) = self;
        let r = p.parse_only(s, o);

        match r.v {
            Some(_) => response(Some(()), r.o, r.c),
            None if r.c == false => response(Some(()), o, r.c),
            None => response(None, o, r.c)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, E, A> Executable<'a, Vec<A>> for Repeat<E, A>
    where E: Executable<'a, A> + Parser<A>
{
    fn execute(&self, s: &'a [u8], o: usize) -> Response<Vec<A>> {
        let Repeat(opt, p, _) = self;

        let mut result: Vec<A> = Vec::with_capacity(13);
        let mut offset = o;
        let mut consumed = false;

        loop {
            let r = p.execute(s, offset);
            match r.v {
                Some(a) => {
                    result.push(a);
                    offset = r.o;
                    consumed = consumed || r.c;
                }
                _ => {
                    if *opt || offset - o > 0 {
                        return response(Some(result), offset, consumed);
                    }

                    return response(None, offset, consumed);
                }
            }
        }
    }
}

impl<'a, E, A> Parsable<'a, Vec<A>> for Repeat<E, A>
    where E: Parsable<'a, A> + Parser<A>
{
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let Repeat(opt, p, _) = self;

        let mut offset = o;
        let mut consumed = false;

        loop {
            let r = p.parse_only(s, offset);
            match r.v {
                Some(_) => {
                    offset = r.o;
                    consumed = consumed || r.c;
                }
                _ => {
                    if *opt || offset - o > 0 {
                        return response(Some(()), offset, consumed);
                    }

                    return response(None, offset, consumed);
                }
            }
        }
    }
}
// -------------------------------------------------------------------------------------------------


impl<'a> Executable<'a, ()> for Skip {
    #[inline]
    fn execute(&self, s: &'a [u8], o: usize) -> Response<()> {
        self.parse_only(s, o)
    }
}

impl<'a> Parsable<'a, ()> for Skip {
    #[inline]
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let Skip(chars) = self;
        let bytes = chars.as_bytes();
        let mut n = o;

        while n < s.len() && bytes.contains(&s[n]) {
            n += 1;
        }

        response(Some(()), n, false)
    }
}

// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! seq {
    (($l:expr))              => { $l                          };
    (($l:expr) <~ $($r:tt)+) => { $l.then_left(seq!($($r)+))  };
    (($l:expr) ~> $($r:tt)+) => { $l.then_right(seq!($($r)+)) };
    (($l:expr) ~  $($r:tt)+) => { $l.then(seq!($($r)+))       };
    (($l:expr) >> $r:expr)   => { $l.fmap(Box::new($r))       };
}

#[macro_export]
macro_rules! cases {
    (($l:expr))              => { $l                          };
    (($l:expr) |  $($r:tt)+) => { $l.or(cases!($($r)+))       };
    (($l:expr) >> $r:expr)   => { $l.fmap(Box::new($r))       };
}

// -------------------------------------------------------------------------------------------------
