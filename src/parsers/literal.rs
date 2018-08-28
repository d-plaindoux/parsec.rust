use parsers::basic::*;
use parsers::core::*;
use parsers::execution::*;
use parsers::flow::*;
use parsers::monadic::*;
use parsers::parser::*;
use parsers::response::*;

//  -------------------------------------------------------------------------------------------------
// Parser type definition
//  -------------------------------------------------------------------------------------------------

impl Parser<u8> for u8 {}

impl Parser<char> for char {}

impl Parser<String> for String {}

pub struct DelimitedString();

impl Parser<String> for DelimitedString {}

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


pub fn float() -> Parsec<f32> {
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

    parsec(Box::new(p))
}

pub fn delimited_string() -> DelimitedString {
    /* Not optimized version
    pub fn delimited_string() -> Parsec<String> {
        let p = '"'.
            then_right("\\\"".to_string().fmap(Box::new(|_| '\"')).or(take_one(Box::new(|c| *c != '"' as u8)).fmap(Box::new(|a| a as char))).optrep())
            .then_left('"')
            .fmap(Box::new(|b| b.into_iter().collect::<String>()));

        parsec(Box::new(p))
    }
    */
    DelimitedString()
}

pub fn delimited_char() -> Parsec<char> {
    let p = '\''
        .then_right("\\\'".to_string().fmap(Box::new(|_| '\'')).or(take_one(Box::new(|c| *c != '\'' as u8)).fmap(Box::new(|a| a as char))))
        .then_left('\'');

    parsec(Box::new(p))
}

// -------------------------------------------------------------------------------------------------
// Parser execution
// -------------------------------------------------------------------------------------------------

impl Executable<u8> for u8 {
    fn execute(&self, s: &[u8], o: usize) -> Response<u8> {
        let r = any().execute(s, o);
        match r {
            Response::Success(a, _, _) if { *self == a } => r,
            _ => Response::Reject(o, false)
        }
    }
}

impl Executable<char> for char {
    fn execute(&self, s: &[u8], o: usize) -> Response<char> {
        let r = any().execute(s, o); // TODO unicode to be managed here
        match r {
            Response::Success(a, o, b) if { *self == a as char } => Response::Success(a as char, o, b),
            _ => Response::Reject(o, false)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<String> for String {
    fn execute(&self, s: &[u8], o: usize) -> Response<String> {
        if o + self.len() > s.len() {
            return Response::Reject(o, false);
        }

        let b = self.as_bytes();

        for i in 0..self.len() {
            if s[i + o] != b[i] {
                return Response::Reject(o, false);
            }
        }

        Response::Success(self.clone(), o + self.len(), self.len() > 0)
    }
}

// -------------------------------------------------------------------------------------------------

impl Executable<String> for DelimitedString {
    fn execute(&self, s: &[u8], o: usize) -> Response<String> {
        if o >= s.len() || s[o] as char != '"' {
            return Response::Reject(o, false);
        }

        let mut n = o + 1;
        let mut not_escaped = true;

        while n < s.len() {
            if s[n] as char == '"' && not_escaped {
                return Response::Success(String::from_utf8_lossy(&s[o + 1..n]).to_string(), n + 1, true);
            }

            if s[n] as char == '\\' && not_escaped {
                not_escaped = false;
            } else {
                not_escaped = true;
            }

            n += 1;
        }

        return Response::Reject(n, true);
    }
}
