use parsers::basic::*;
use parsers::core::*;
use parsers::execution::*;
use parsers::flow::*;
use parsers::monadic::*;
use parsers::parser::*;
use parsers::response::Response;

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

pub fn string_delim() -> Parsec<String> {
    let p = '"'.
        then_right("\\\"".to_string().fmap(Box::new(|_| '\"')).or(take_one(Box::new(|c| *c != '"'))).optrep())
        .then_left('"')
        .fmap(Box::new(|b| b.into_iter().collect::<String>()));

    parsec(Box::new(p))
}

pub fn char_delim() -> Parsec<char> {
    let p = '\''
        .then_right("\\\'".to_string().fmap(Box::new(|_| '\'')).or(take_one(Box::new(|c| *c != '\''))))
        .then_left('\'');

    parsec(Box::new(p))
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
