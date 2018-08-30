use parsers::basic::*;
use parsers::data::*;
use parsers::execution::*;
use parsers::flow::*;
use parsers::monadic::*;
use parsers::parser::*;
use parsers::response::*;
use std::ops::Deref;
use std::ops::Range;

//  -------------------------------------------------------------------------------------------------
// Parser type definition
//  -------------------------------------------------------------------------------------------------

impl Parser<u8> for u8 {}

impl Parser<char> for char {}

impl Parser<char> for Range<char> {}

impl Parser<String> for String {}

impl<'a> Parser<&'a str> for &'a str {}

pub struct Float();

impl<'a> Parser<FloatLiteral<'a>> for Float {}

pub struct DelimitedString();

impl<'a> Parser<StringLiteral<'a>> for DelimitedString {}

pub struct DelimitedChar();

impl Parser<char> for DelimitedChar {}


//  -------------------------------------------------------------------------------------------------

pub fn digit() -> Range<char> {
    '0'..'9'
}

pub fn letter() -> Or<Range<char>, Range<char>, char> {
    ('a'..'z').or('A'..'Z')
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
        match r.v {
            Some(a) if { *self == a } => r,
            _ => response(None, o, false)
        }
    }
}

impl<'a> Parsable<'a, u8> for u8 {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let r = any().execute(s, o);
        match r.v {
            Some(a) if { *self == a } => response(Some(()), r.o, r.c),
            _ => response(None, o, false)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, char> for char {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<char> {
        let r = any().execute(s, o); // TODO unicode to be managed here
        match r.v {
            Some(a) if { *self == a as char } => response(Some(a as char), r.o, r.c),
            _ => response(None, o, false)
        }
    }
}

impl<'a> Parsable<'a, char> for char {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let r = any().execute(s, o); // TODO unicode to be managed here
        match r.v {
            Some(a) if { *self == a as char } => response(Some(()), r.o, r.c),
            _ => response(None, o, false)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, char> for Range<char> {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<char> {
        let r = any().execute(s, o); // TODO unicode to be managed here
        match r.v {
            Some(a) => {
                let c = a as char;
                if c >= self.start && c <= self.end {
                    response(Some(a as char), r.o, r.c)
                } else {
                    response(None, o, false)
                }
            }
            _ => response(None, o, false)
        }
    }
}

impl<'a> Parsable<'a, char> for Range<char> {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let r = any().execute(s, o); // TODO unicode to be managed here
        match r.v {
            Some(a) => {
                let c = a as char;
                if c >= self.start && c <= self.end {
                    response(Some(()), r.o, r.c)
                } else {
                    response(None, o, false)
                }
            }
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

impl<'a> Parsable<'a, String> for String {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let r = self.deref().execute(s, o);
        match r.v {
            Some(_) => response(Some(()), r.o, r.c),
            _ => response(None, r.o, r.c)
        }
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

impl<'a> Executable<'a, FloatLiteral<'a>> for Float {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<FloatLiteral<'a>> {
        let p = '+'.or('-').opt()
            .then(('0'..'9').rep())
            .then('.'.then(('0'..'9').rep()).opt());

        let r = p.parse_only(s, o);

        match r.v {
            Some(_) => response(Some(FloatLiteral(s, o, r.o)), r.o, r.c),
            _ => response(None, r.o, r.c)
        }
    }
}

impl<'a> Parsable<'a, FloatLiteral<'a>> for Float {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let p = '+'.or('-').opt()
            .then(('0'..'9').rep())
            .then('.'.then(('0'..'9').rep()).opt());

        let r = p.parse_only(s, o);

        match r.v {
            Some(_) => response(Some(()), r.o, r.c),
            _ => response(None, r.o, r.c)
        }
    }
}
// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, StringLiteral<'a>> for DelimitedString {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<StringLiteral<'a>> {
        let c = '\\'.then_right(any()).or(any().satisfy(Box::new(|b| *b as char != '"')));
        let p = '"'.then(c.optrep()).then('"');

        let r = p.parse_only(s, o);

        match r.v {
            Some(_) => response(Some(StringLiteral(s, o + 1, r.o - 1)), r.o, r.c),
            _ => response(None, r.o, r.c)
        }
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
