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
            _ => Response::Reject(o, false)
        }
    }
}

impl ParserTrait<String> for String {
    fn do_parse(&self, s: &str, o: usize) -> Response<String> {
        if o + self.len() > s.len() || unsafe { s.slice_unchecked(o, o + self.len()) } != self {
            return Response::Reject(o, false);
        }

        Response::Success(self.get(o..o + self.len()).unwrap().to_string(), o + self.len(), self.len() > 0)
    }
}

pub fn digit() -> Parser<char> {
    parser!(
        take_one!(|a| {
            match *a {
                '0'...'9' => true,
                _ => false
            }
        })
    )
}

pub fn letter() -> Parser<char> {
    parser!(
        take_one!(|a| {
            match *a {
                'a'...'z' => true,
                'A'...'Z' => true,
                _ => false
            }
        })
    )
}

pub fn natural() -> Parser<i32> {
    parser!(
        fmap!(
            |(a,b):(Option<char>, Vec<char>)| {
                let result = b.into_iter().collect::<String>().parse::<i32>().unwrap();

                match a {
                    Some('-') => -1 * result,
                    _ => result
                }
            },
            then!(opt!(or!('+','-')), rep!(digit()))
        )
    )
}

pub fn string_delim() -> Parser<String> {
    parser!(
        fmap!(
            |(_,(b,_)):(char, (Vec<char>, char))| b.into_iter().collect::<String>(),
            then!(
                '"',
                optrep!(
                    or!(
                        fmap!(|_| '\"', "\\\"".to_string()),
                        take_one!(|c| *c != '"')
                    )
                ),
                '"'
            )
        )
    )
}

pub fn char_delim() -> Parser<char> {
    parser!(
        fmap!(
            |(_,(b,_)):(char, (char, char))| b,
            then!(
                '\'',
                or!(
                    fmap!(|_| '\'', "\\\'".to_string()),
                    take_one!(|c| *c != '\'')
                ),
                '\''
            )
        )
    )
}