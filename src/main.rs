fn main() {}

// -------------------------------------------------------------------------------------------------
// Response definition
// -------------------------------------------------------------------------------------------------

pub enum Response<A> {
    Success(A, String, bool),
    Reject(bool),
}

pub fn fold<A, B>(s: Response<A>, success: fn(A, String, bool) -> B, reject: fn(bool) -> B) -> B {
    match s {
        Response::Success(a, s, b) => success(a, s, b),
        Response::Reject(b) => reject(b)
    }
}


// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub trait Parser<A> {
    fn parse(&self, s: String) -> Response<A>;
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

fn returns<A>(a: A) -> Returns<A> {
    return Returns { a };
}

pub struct Fails;

impl<A:> Parser<A> for Fails {
    fn parse(&self, _: String) -> Response<A> {
        return Response::Reject(false);
    }
}

pub fn fails() -> Fails {
    return Fails {};
}

// -------------------------------------------------------------------------------------------------
// Monadic
// -------------------------------------------------------------------------------------------------

pub struct Join<A> { p: Box<Parser<Box<Parser<A>>>> } // How this Box of Box can be simplified ?

impl<A> Parser<A> for Join<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p.parse(s) {
            Response::Reject(b1) => Response::Reject(b1),
            Response::Success(a1, i1, b1) => {
                match a1.parse(i1.to_string()) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2),
                }
            }
        }
    }
}

pub fn join<A>(p: Box<Parser<Box<Parser<A>>>>) -> Join<A> {
    return Join { p };
}

// -------------------------------------------------------------------------------------------------

pub struct FMap<A, B> { f: fn(A) -> B, p: Box<Parser<A>> } // Can we remove this Box

impl<A, B> Parser<B> for FMap<A, B> {
    fn parse(&self, s: String) -> Response<B> {
        match self.p.parse(s) {
            Response::Success(a, i, b) => Response::Success((self.f)(a), i, b),
            Response::Reject(b) => Response::Reject(b)
        }
    }
}

pub fn fmap<A, B>(f: fn(A) -> B, p: Box<Parser<A>>) -> FMap<A, B> {
    return FMap { f, p };
}

// -------------------------------------------------------------------------------------------------

pub struct Bind<A, B> { f: fn(A) -> Box<Parser<B>>, p: Box<Parser<A>> } // Can we remove this Box

impl<A, B> Parser<B> for Bind<A, B> {
    fn parse(&self, s: String) -> Response<B> {
        // return join(Box::new(fmap(self.f, self.p))).parse(s); ???
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

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests_parsec {
    #[test]
    fn it_returns() {
        use super::*;

        let r = returns(1);

        assert_eq!(1, fold(
            r.parse("a".to_string()),
            |a, _, _| a,
            |_| 0,
        ));
    }

    #[test]
    fn it_fails() {
        use super::*;

        let r = fails();
        assert_eq!(0, fold(
            r.parse("a".to_string()),
            |a, _, _| a,
            |_| 0,
        ));
    }

    #[test]
    fn it_fmap() {
        use super::*;

        let p = Box::new(returns(1));
        let r = fmap(|a: u32| a.to_string(), p);
        assert_eq!("1".to_string(), fold(
            r.parse("a".to_string()),
            |a, _, _| a,
            |_| "0".to_string(),
        ));
    }

    #[test]
    fn it_bind() {
        use super::*;

        let p = Box::new(returns(1));
        let r = Bind { f: |a: u32|Box::new(returns(a + 1)), p };
        assert_eq!(2, fold(
            r.parse("a".to_string()),
            |a, _, _| a,
            |_| 0,
        ));
    }
}
