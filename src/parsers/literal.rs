use parsers::basic::*;
use parsers::core::*;
use parsers::flow::*;
use parsers::monadic::*;
use parsers::response::Response;
use std::prelude::v1::Vec;

//  -------------------------------------------------------------------------------------------------
// Parser type definition
//  -------------------------------------------------------------------------------------------------

impl Parser<char> for char {}

impl Parser<String> for String {}

pub fn digit() -> TakeOne {
    take_one(Box::new(|a| {
        match *a {
            '0'...'9' => true,
            _ => false
        }
    }))
}

pub fn letter() -> TakeOne {
    take_one(Box::new(|a| {
        match *a {
            'a'...'z' => true,
            'A'...'Z' => true,
            _ => false
        }
    }))
}

type Natural = FMap<And<Or<FMap<Or<char, char, char>, char, Option<char>>, Return<Option<char>>, Option<char>>, Option<char>, Repeat<Satisfy<Try<Any, char>, char>, char>, Vec<char>>, (Option<char>, Vec<char>), i32>;

pub fn natural() -> Natural {
    opt('+'.or('-')).then(digit().rep()).fmap(Box::new(|(a, b): (Option<char>, Vec<char>)| {
        let result = b.into_iter().collect::<String>().parse::<i32>().unwrap();

        match a {
            Some('-') => -1 * result,
            _ => result
        }
    }))
}

pub type DelimitedString = FMap<And<And<char, char, Repeat<Or<FMap<String, String, char>, Satisfy<Try<Any, char>, char>, char>, char>, Vec<char>>, (char, Vec<char>), char, char>, ((char, Vec<char>), char), String>;

pub fn string_delim() -> DelimitedString {
    '"'.then("\\\"".to_string().fmap(Box::new(|_| '\"')).or(take_one(Box::new(|c| *c != '"'))).optrep())
        .then('"')
        .fmap(Box::new(|((_, b), _): ((char, Vec<char>), char)| b.into_iter().collect::<String>()))
}

pub type DelimitedChar = FMap<And<And<char, char, Or<FMap<String, String, char>, Satisfy<Try<Any, char>, char>, char>, char>, (char, char), char, char>, ((char, char), char), char>;

pub fn char_delim() -> DelimitedChar {
    '\''.then("\\\'".to_string().fmap(Box::new(|_| '\'')).or(take_one(Box::new(|c| *c != '\''))))
        .then('\'')
        .fmap(Box::new(|((_, b), _): ((char, char), char)| b))
}

// -------------------------------------------------------------------------------------------------
// Parser execution
// -------------------------------------------------------------------------------------------------

impl Executable<char> for char {
    fn execute(&self, s: &str, o: usize) -> Response<char> {
        let r = any().execute(s, o);
        match r {
            Response::Success(a, _, _) if { *self == a } => r,
            _ => Response::Reject(o, false)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<String> for String {
    fn execute(&self, s: &str, o: usize) -> Response<String> {
        if o + self.len() > s.len() || unsafe { s.slice_unchecked(o, o + self.len()) } != self {
            return Response::Reject(o, false);
        }

        Response::Success(self.get(o..o + self.len()).unwrap().to_string(), o + self.len(), self.len() > 0)
    }
}

// -------------------------------------------------------------------------------------------------
