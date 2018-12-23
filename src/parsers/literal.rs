use std::ops::Deref;
use std::ops::Range;

use crate::parsers::basic::*;
use crate::parsers::execution::*;
use crate::parsers::flow::*;
use crate::parsers::monadic::*;
use crate::parsers::parser::*;
use crate::parsers::response::*;

//  -------------------------------------------------------------------------------------------------
// Parser type definition
//  -------------------------------------------------------------------------------------------------

impl Parser<u8> for u8 {}

impl Parser<char> for char {}

impl Parser<char> for Range<char> {}

impl Parser<String> for String {}

impl<'a> Parser<&'a str> for &'a str {}

pub struct Float();

impl<'a> Parser<&'a [u8]> for Float {}

pub struct DelimitedString();

impl<'a> Parser<&'a [u8]> for DelimitedString {}

pub struct DelimitedChar();

impl Parser<char> for DelimitedChar {}

//  -------------------------------------------------------------------------------------------------

pub fn digit() -> Range<char> {
    '0'..'9'
}

pub fn letter() -> Or<Range<char>, Range<char>, char> {
    ('a'..'z').or('A'..'Z')
}

pub fn float() -> Float {
    Float()
}

pub fn delimited_string() -> DelimitedString {
    DelimitedString()
}

pub fn delimited_char() -> DelimitedChar {
    DelimitedChar()
}

// -------------------------------------------------------------------------------------------------
// Parser execution
// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, u8> for u8 {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<u8> {
        let result = any().execute(s, o);
        match result.v {
            Some(value) if { *self == value } => result,
            _ => response(None, o, false),
        }
    }
}

impl<'a> Parsable<'a, u8> for u8 {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let result = any().execute(s, o);
        match result.v {
            Some(value) if { *self == value } => response(Some(()), result.o, result.c),
            _ => response(None, o, false),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, char> for char {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<char> {
        let result = any().execute(s, o); // TODO unicode to be managed here
        match result.v {
            Some(value) if { *self == value as char } => response(Some(value as char), result.o, result.c),
            _ => response(None, o, false),
        }
    }
}

impl<'a> Parsable<'a, char> for char {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let result = any().execute(s, o); // TODO unicode to be managed here
        match result.v {
            Some(value) if { *self == value as char } => response(Some(()), result.o, result.c),
            _ => response(None, o, false),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, char> for Range<char> {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<char> {
        let result = any().execute(s, o); // TODO unicode to be managed here
        match result.v {
            Some(value) => {
                let c = value as char;
                if c >= self.start && c <= self.end {
                    response(Some(value as char), result.o, result.c)
                } else {
                    response(None, o, false)
                }
            }
            _ => response(None, o, false),
        }
    }
}

impl<'a> Parsable<'a, char> for Range<char> {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let result = any().execute(s, o); // TODO unicode to be managed here
        match result.v {
            Some(value) => {
                let c = value as char;
                if c >= self.start && c <= self.end {
                    response(Some(()), result.o, result.c)
                } else {
                    response(None, o, false)
                }
            }
            _ => response(None, o, false),
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
        self.deref().parse_only(s, o)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, &'a str> for &'a str {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<&'a str> {
        let r = self.parse_only(s, o);

        match r.v {
            Some(_) => response(Some(self), r.o, r.c),
            _ => response(None, r.o, r.c),
        }
    }
}

impl<'a> Parsable<'a, &'a str> for &'a str {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        if o + self.len() > s.len() {
            return response(None, o, false);
        }

        let b = self.as_bytes();

        for i in 0..self.len() {
            if s[i + o] != b[i] {
                return response(None, o, false);
            }
        }

        response(Some(()), o + self.len(), !self.is_empty())
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, &'a [u8]> for Float {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<&'a [u8]> {
        let r = self.parse_only(s, o);

        match r.v {
            Some(_) => response(Some(&s[o..r.o]), r.o, r.c),
            _ => response(None, r.o, r.c),
        }
    }
}

impl<'a> Parsable<'a, &'a [u8]> for Float {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let p = '+'
            .or('-')
            .opt()
            .then(('0'..'9').rep())
            .then('.'.then(('0'..'9').rep()).opt());

        p.parse_only(s, o)
    }
}
// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, &'a [u8]> for DelimitedString {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<&'a [u8]> {
        let r = self.parse_only(s, o);

        match r.v {
            Some(_) => response(Some(&s[o + 1..r.o - 1]), r.o, r.c),
            _ => response(None, r.o, r.c),
        }
    }
}

impl<'a> Parsable<'a, &'a [u8]> for DelimitedString {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        let c = '\\'
            .then_right(any())
            .or(any().satisfy(|b| *b as char != '"'));
        let p = '"'.then(c.optrep()).then('"');

        p.parse_only(s, o)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Executable<'a, char> for DelimitedChar {
    fn execute(&self, s: &'a [u8], o: usize) -> Response<char> {
        let p = '\''
            .then_right(
                "\\\'"
                    .fmap(|_| '\'')
                    .or(take_one(|c| *c != b'\'').fmap(|a| a as char)),
            )
            .then_left('\'');

        p.execute(s, o)
    }
}

impl<'a> Parsable<'a, char> for DelimitedChar {
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()> {
        self.execute(s, o).fmap(|_| ())
    }
}
