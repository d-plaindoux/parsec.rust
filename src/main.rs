use response::*;
use std::collections::LinkedList;

fn main() {}

// -------------------------------------------------------------------------------------------------
// Response definition
// -------------------------------------------------------------------------------------------------

mod response {
    pub enum Response<A> {
        Success(A, String, bool),
        Reject(bool),
    }

    type OnSuccess<A, B> = fn(A, String, bool) -> B;
    type OnReject<B> = fn(bool) -> B;

    pub fn fold<A, B>(s: Response<A>, success: OnSuccess<A, B>, reject: OnReject<B>) -> B {
        match s {
            Response::Success(a, s, b) => success(a, s, b),
            Response::Reject(b) => reject(b)
        }
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

// -------------------------------------------------------------------------------------------------

pub struct Fails;

impl<A> Parser<A> for Fails {
    fn parse(&self, _: String) -> Response<A> {
        return Response::Reject(false);
    }
}

pub fn fails() -> Fails {
    return Fails {};
}

// -------------------------------------------------------------------------------------------------

pub struct Any;

impl Parser<char> for Any {
    fn parse(&self, s: String) -> Response<char> {
        if s.len() < 1 {
            return Response::Reject(false);
        }

        return Response::Success(s.chars().next().unwrap(), s[1..].to_string(), true);
    }
}

pub fn any() -> Any {
    return Any {};
}

// -------------------------------------------------------------------------------------------------

pub struct Try<A> { p: Box<Parser<A>> }

impl<A> Parser<A> for Try<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p.parse(s) {
            Response::Reject(_) => Response::Reject(false),
            r => r
        }
    }
}

pub fn try<A>(p: Box<Parser<A>>) -> Try<A> {
    Try { p }
}

// -------------------------------------------------------------------------------------------------

pub struct Lookahead<A> { p: Box<Parser<A>> }

impl<A> Parser<A> for Lookahead<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p.parse(s.clone()) {
            Response::Success(a, _, b) => Response::Success(a, s, b),
            _ => Response::Reject(false),
        }
    }
}

pub fn lookahead<A>(p: Box<Parser<A>>) -> Lookahead<A> {
    Lookahead { p }
}

// -------------------------------------------------------------------------------------------------
// Monadic
// -------------------------------------------------------------------------------------------------

pub struct Join<A> { p: Box<Parser<Box<Parser<A>>>> } // How this Box of Box can be simplified ?

impl<A> Parser<A> for Join<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p.parse(s) {
            Response::Success(a1, i1, b1) => {
                match a1.parse(i1.to_string()) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2),
                }
            }
            Response::Reject(b1) => Response::Reject(b1)
        }
    }
}

pub fn join<A>(p: Box<Parser<Box<Parser<A>>>>) -> Join<A> {
    Join { p }
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
    FMap { f, p }
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

pub fn bind<A, B>(f: fn(A) -> Box<Parser<B>>, p: Box<Parser<A>>) -> Bind<A, B> {
    Bind { f, p }
}

// -------------------------------------------------------------------------------------------------
// Flow
// -------------------------------------------------------------------------------------------------

pub struct And<A, B> { p1: Box<Parser<A>>, p2: Box<Parser<B>> }

impl<A, B> Parser<(A, B)> for And<A, B> {
    fn parse(&self, s: String) -> Response<(A, B)> {
        match self.p1.parse(s) {
            Response::Success(a1, i1, b1) => {
                match self.p2.parse(i1.to_string()) {
                    Response::Success(a2, i2, b2) => Response::Success((a1, a2), i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2),
                }
            }
            Response::Reject(b1) => Response::Reject(b1)
        }
    }
}

pub fn and<A, B>(p1: Box<Parser<A>>, p2: Box<Parser<B>>) -> And<A, B> {
    And { p1, p2 }
}

// -------------------------------------------------------------------------------------------------

pub struct Or<A> { p1: Box<Parser<A>>, p2: Box<Parser<A>> }

impl<A> Parser<A> for Or<A> {
    fn parse(&self, s: String) -> Response<A> {
        match self.p1.parse(s.clone()) { // Borrowing ...
            Response::Success(a1, i1, b1) => Response::Success(a1, i1, b1),
            Response::Reject(b1) => {
                match self.p2.parse(s) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2)
                }
            }
        }
    }
}

pub fn or<A>(p1: Box<Parser<A>>, p2: Box<Parser<A>>) -> Or<A> {
    Or { p1, p2 }
}

// -------------------------------------------------------------------------------------------------
// Occurrences
// -------------------------------------------------------------------------------------------------

pub struct Opt<A> { p: Box<Parser<A>> }

impl<A> Parser<Option<A>> for Opt<A> {
    fn parse(&self, s: String) -> Response<Option<A>> {
        match self.p.parse(s.clone()) { // Borrowing ...
            Response::Success(a1, i1, b1) => Response::Success(Some(a1), i1, b1),
            _ => Response::Success(None, s, false)
        }
    }
}

pub fn opt<A>(p: Box<Parser<A>>) -> Opt<A> {
    Opt { p }
}

// -------------------------------------------------------------------------------------------------

pub struct Repeat<A> { opt: bool, p: Box<Parser<A>> }

impl<A> Parser<LinkedList<A>> for Repeat<A> {
    fn parse(&self, s: String) -> Response<LinkedList<A>> {
        let mut result: LinkedList<A> = Default::default();
        let mut input = s;
        let mut consumed = false;
        let mut parsed = true;

        while parsed {
            match self.p.parse(input.clone()) {
                Response::Success(a1, i1, b1) => {
                    result.push_back(a1);
                    input = i1;
                    consumed = consumed || b1;
                }
                _ => {
                    parsed = false;
                }
            }
        }

        if self.opt || result.len() > 0 {
            return Response::Success(result, input, consumed);
        }

        return Response::Reject(consumed);
    }
}

pub fn optrep<A>(p: Box<Parser<A>>) -> Repeat<A> {
    Repeat { opt: true, p }
}

pub fn rep<A>(p: Box<Parser<A>>) -> Repeat<A> {
    Repeat { opt: false, p }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests_parsec {
    use super::*;

    #[test]
    fn it_parse_with_returns() {
        let r = returns(1);

        assert_eq!(1, fold(
            r.parse("a".to_string()),
            |a: u32, _, _| a,
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_fails() {
        let r = fails();
        assert_eq!(0, fold(
            r.parse("a".to_string()),
            |_: u32, _, _| panic!("Parse error"),
            |_| 0,
        ));
    }

    #[test]
    fn it_parse_with_any_success() {
        let r = any();
        assert_eq!('a', fold(
            r.parse("a".to_string()),
            |a, _, _| a,
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_try_any_reject() {
        let r = try(Box::new(any()));
        assert_eq!(false, fold(
            r.parse("".to_string()),
            |_, _, _| panic!("Parse error"),
            |b| b,
        ));
    }

    #[test]
    fn it_parse_with_try_any_success() {
        let r = try(Box::new(any()));
        assert_eq!(true, fold(
            r.parse("a".to_string()),
            |_, _, b| b,
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_lookahead_any_reject() {
        let r = lookahead(Box::new(any()));
        assert_eq!(false, fold(
            r.parse("".to_string()),
            |_, _, _| panic!("Parse error"),
            |b| b,
        ));
    }

    #[test]
    fn it_parse_with_lookahead_any_success() {
        let r = lookahead(Box::new(any()));
        assert_eq!(true, fold(
            r.parse("a".to_string()),
            |_, _, b| b,
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_fmap_success() {
        let p = Box::new(returns(1));
        let r = fmap(|a| a.to_string(), p);
        assert_eq!("1".to_string(), fold(
            r.parse("a".to_string()),
            |a, _, _| a,
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_fmap_reject() {
        let p = Box::new(fails());
        let r = fmap(|a: u32| a.to_string(), p);
        assert_eq!("0".to_string(), fold(
            r.parse("a".to_string()),
            |_, _, _| panic!("Parse error"),
            |_| "0".to_string(),
        ));
    }

    #[test]
    fn it_parse_with_bind_success() {
        let p = Box::new(returns(1));
        let r = bind(|a| Box::new(returns(a + 1)), p);
        assert_eq!(2, fold(
            r.parse("a".to_string()),
            |a, _, _| a,
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_bind_reject() {
        let p = Box::new(returns(1));
        let r = bind(|_| Box::new(fails()), p);
        assert_eq!(0, fold(
            r.parse("a".to_string()),
            |_: u32, _, _| panic!("Parse error"),
            |_| 0,
        ));
    }

    #[test]
    fn it_parse_with_and() {
        let p1 = Box::new(any());
        let p2 = Box::new(any());
        let r = and(p1, p2);
        assert_eq!(('a', 'b'), fold(
            r.parse("ab".to_string()),
            |a, _, _| a,
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_or_success() {
        let p1 = Box::new(returns(2));
        let p2 = Box::new(fails());
        let r = or(p1, p2);
        assert_eq!(2, fold(
            r.parse("a".to_string()),
            |a, _, _| a,
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_or_reject() {
        let p1 = Box::new(fails());
        let p2 = Box::new(returns(2));
        let r = or(p1, p2);
        assert_eq!(2, fold(
            r.parse("a".to_string()),
            |a, _, _| a,
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_optrep_success() {
        let p1 = Box::new(any());
        let r = optrep(p1);
        let s = 1024 * 64;
        assert_eq!(s, fold(
            r.parse("a".repeat(s).to_string()),
            |a, _, _| a.len(),
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_optrep_success_empty() {
        let p1 = Box::new(any());
        let r = optrep(p1);
        assert_eq!(0, fold(
            r.parse("".to_string()),
            |a, _, _| a.len(),
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_rep_success() {
        let p1 = Box::new(any());
        let r = rep(p1);
        let s = 1024 * 64;
        assert_eq!(s, fold(
            r.parse("a".repeat(s).to_string()),
            |a, _, _| a.len(),
            |_| panic!("Parse error"),
        ));
    }

    #[test]
    fn it_parse_with_rep_reject_empty() {
        let p1 = Box::new(any());
        let r = rep(p1);
        assert_eq!(false, fold(
            r.parse("".to_string()),
            |_, _, _| panic!("Parse error"),
            |b| b,
        ));
    }
}
