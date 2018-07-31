use parsers::basic::*;
use parsers::core::*;
use parsers::flow::*;
use parsers::monadic::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Chars
// -------------------------------------------------------------------------------------------------

impl ParserTrait<char> for char {
    fn do_parse(&self, s: &str, o: usize) -> Response<char> {
        let r = any().do_parse(s, o);
        match r {
            Response::Success(a, _, _) if { *self == a } => r,
            _ => Response::Reject(false)
        }
    }
}

impl ParserTrait<String> for String {
    fn do_parse(&self, s: &str, o: usize) -> Response<String> {
        if o + self.len() > s.len() || unsafe { self.slice_unchecked(o, o + self.len()) } != s {
            return Response::Reject(false);
        }

        Response::Success(self.get(o..o + self.len()).unwrap().to_string(), o + self.len(), self.len() > 0)
    }
}

impl ParserTrait<char> for fn(char) -> bool {
    fn do_parse(&self, s: &str, o: usize) -> Response<char> {
        let r = any().do_parse(s, o);
        match r {
            Response::Success(a, _, _) if { self(a) } => r,
            _ => Response::Reject(false)
        }
    }
}

pub fn digit() -> fn(char) -> bool {
    let p: fn(char) -> bool = |a| match a {
        '0'...'9' => true,
        _ => false
    };

    p
}

pub fn letter() -> fn(char) -> bool {
    let p: fn(char) -> bool = |a| match a {
        'a'...'z' => true,
        'A'...'Z' => true,
        _ => false
    };

    p
}

pub fn natural() -> Parser<i32> {
    parser!(
        fmap!(
            |(a,b):(Option<char>, String)|
                (a.unwrap_or('+').to_string() + b.as_str()).parse::<i32>().unwrap(),
            and!(opt!(or!('+','-')),
                fmap!(
                    |a:Vec<char>| a.into_iter().collect(),
                    rep!(digit())
                )
            )
        )
    )
}