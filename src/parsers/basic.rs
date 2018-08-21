use parsers::core::Executable;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub struct Return<E>(pub E);

pub struct Fail();

pub struct Any();

pub struct Eos();

pub struct Try<E>(pub E);

pub struct Lookahead<E>(pub E);

// -------------------------------------------------------------------------------------------------
// Parser execution
// -------------------------------------------------------------------------------------------------

impl<A> Executable<A> for Return<A>
    where A: Copy
{
    fn execute(&self, _: &str, o: usize) -> Response<A> {
        let Return(v) = self;

        Response::Success(v.clone(), o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl<A> Executable<A> for Fail
{
    fn execute(&self, _: &str, o: usize) -> Response<A> {
        Response::Reject(o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<char> for Any
{
    fn execute(&self, s: &str, o: usize) -> Response<char> {
        if o >= s.len() {
            return Response::Reject(o, false);
        }

        Response::Success(s[o..(o + 1)].chars().next().unwrap(), o + 1, true)
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<()> for Eos
{
    fn execute(&self, s: &str, o: usize) -> Response<()> {
        if o < s.len() {
            return Response::Reject(o, false);
        }

        Response::Success((), o, false)
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Try<E> where E: Executable<A>
{
    fn execute(&self, s: &str, o: usize) -> Response<A> {
        let Try(p) = self;

        match p.execute(s, o) {
            Response::Reject(o, _) => Response::Reject(o, false),
            other => other
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Lookahead<E> where E: Executable<A>
{
    fn execute(&self, s: &str, o: usize) -> Response<A> {
        let Lookahead(p) = self;

        match p.execute(s, o) {
            Response::Success(v, _, _) => Response::Success(v, o, false),
            other => other
        }
    }
}

// -------------------------------------------------------------------------------------------------
