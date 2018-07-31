use parsers::basic::any;
use parsers::core::Parser;
use parsers::response::Response;

// -------------------------------------------------------------------------------------------------
// Chars
// -------------------------------------------------------------------------------------------------

impl Parser<char> for char {
    fn parse(&self, s: &str, o: usize) -> Response<char> {
        let response = any().parse(s, o);
        match response {
            Response::Success(a, _, _) => {
                if a == *self {
                    return response;
                }

                return Response::Reject(false);
            }
            r => r
        }
    }
}

impl Parser<String> for String {
    fn parse(&self, s: &str, o: usize) -> Response<String> {
        if o + self.len() > s.len() || unsafe { self.slice_unchecked(o, o + self.len()) } != s {
            return Response::Reject(false);
        }

        return Response::Success(self.get(o..o + self.len()).unwrap().to_string(), o + self.len(), self.len() > 0);
    }
}

impl Parser<char> for fn(char) -> bool {
    fn parse(&self, s: &str, o: usize) -> Response<char> {
        let response = any().parse(s, o);
        match response {
            Response::Success(a, _, _) => {
                if self(a) {
                    return response;
                }

                return Response::Reject(false);
            }
            r => r
        }
    }
}
