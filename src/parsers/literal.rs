use parsers::basic::*;
use parsers::data::SubString;
use parsers::execution::*;
use parsers::flow::*;
use parsers::monadic::*;
use parsers::parser::*;
use parsers::response::*;
use std::ops::Deref;

//  -------------------------------------------------------------------------------------------------
// Parser type definition
//  -------------------------------------------------------------------------------------------------

impl Parser<u8> for u8 {}

impl Parser<char> for char {}

impl Parser<String> for String {}

impl<'a> Parser<&'a str> for &'a str {}

pub struct Float();

impl Parser<f32> for Float {}

pub struct DelimitedString();

impl<'a> Parser<SubString<'a>> for DelimitedString {}

pub struct DelimitedChar();

impl Parser<char> for DelimitedChar {}


//  -------------------------------------------------------------------------------------------------

pub fn digit() -> FMap<TakeOne, u8, char> {
    take_one(Box::new(|a| {
        match *a as char {
            '0'...'9' => true,
            _ => false
        }
    })).fmap(Box::new(|a| a as char))
}

pub fn letter() -> FMap<TakeOne, u8, char> {
    take_one(Box::new(|a| {
        match *a as char {
            'a'...'z' => true,
            'A'...'Z' => true,
            _ => false
        }
    })).fmap(Box::new(|a| a as char))
}


pub fn float<'a>() -> Float {
    Float()
}

pub fn delimited_string() -> DelimitedString {
    DelimitedString()
}

pub fn delimited_char<'a>() -> DelimitedChar {
    DelimitedChar()
}

// -------------------------------------------------------------------------------------------------
// Parser execution
// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, u8> for u8 {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<u8> {
        let r = any().execute(s, o);
        match r {
            Response(Some(a), _, _) if { *self == a } => r,
            _ => response(None, o, false)
        }
    }
}

impl<'a> Executable<'a, char> for char {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<char> {
        let r = any().execute(s, o); // TODO unicode to be managed here
        match r {
            Response(Some(a), o, b) if { *self == a as char } => response(Some(a as char), o, b),
            _ => response(None, o, false)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, String> for String {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<String> {
        self.deref().execute(s, o).fmap(|s| s.to_string())
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, &'a str> for &'a str {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<&'a str> {
        if o + self.len() > s.len() {
            return response(None, o, false);
        }

        let b = self.as_bytes();

        for i in 0..self.len() {
            if s[i + o] != b[i] {
                return response(None, o, false);
            }
        }

        response(Some(self), o + self.len(), self.len() > 0)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, f32> for Float {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<f32> {
        let p = opt('+'.or('-'))
            .then(digit().rep())
            .then(opt('.'.then_right(digit().rep())))
            .fmap(Box::new(|((a, b), c)| {
                let mut number = b.into_iter().collect::<String>();

                if let Some(v) = c {
                    number.push_str(".");
                    number.push_str(&v.into_iter().collect::<String>());
                };

                let result = number.parse::<f32>().unwrap();

                match a {
                    Some('-') => -1f32 * result,
                    _ => result
                }
            }));

        p.execute(s, o)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, SubString<'a>> for DelimitedString {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<SubString<'a>> {
        if o >= s.len() || s[o] as char != '"' {
            return response(None, o, false);
        }

        let mut n = o + 1;
        let mut not_escaped = true;

        while n < s.len() {
            let c = s[n] as char;

            if c == '"' {
                if not_escaped {
                    //String::from_utf8_lossy(&s[o + 1..n]).to_string()
                    return response(Some(SubString(s, o, n)), n + 1, true);
                }
            }

            not_escaped = (c != '\\') || !not_escaped;

            n += 1;
        }

        response(None, n, true)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, char> for DelimitedChar {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<char> {
        let p = '\''
            .then_right("\\\'".to_string().fmap(Box::new(|_| '\'')).or(take_one(Box::new(|c| *c != '\'' as u8)).fmap(Box::new(|a| a as char))))
            .then_left('\'');

        p.execute(s, o)
    }
}
