use parsers::basic::any;
use parsers::core::Parser;
use parsers::response::Response;

// -------------------------------------------------------------------------------------------------
// Chars
// -------------------------------------------------------------------------------------------------

impl Parser<char> for char {
    fn parse(&self, s: &str, o: usize) -> Response<char> {
        let r = any().parse(s, o);
        match r {
            Response::Success(a, _, _) if { *self == a } => r,
            _ => Response::Reject(false)
        }
    }
}

impl Parser<String> for String {
    fn parse(&self, s: &str, o: usize) -> Response<String> {
        if o + self.len() > s.len() || unsafe { self.slice_unchecked(o, o + self.len()) } != s {
            return Response::Reject(false);
        }

        Response::Success(self.get(o..o + self.len()).unwrap().to_string(), o + self.len(), self.len() > 0)
    }
}

impl Parser<char> for fn(char) -> bool {
    fn parse(&self, s: &str, o: usize) -> Response<char> {
        let r = any().parse(s, o);
        match r {
            Response::Success(a, _, _) if { self(a) } => r,
            _ => Response::Reject(false)
        }
    }
}


