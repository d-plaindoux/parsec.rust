pub struct Response<A> {
    // None for reject and Some(?) for a success
    pub v: Option<A>,
    pub o: usize,
    pub c: bool,
}

#[inline]
pub fn response<A>(v: Option<A>, o: usize, c: bool) -> Response<A> {
    Response { v, o, c }
}

pub trait FoldResponse<A, B> {
    fn fold_fn(self, success: &Fn(A, usize, bool) -> B, reject: &Fn(usize, bool) -> B) -> B;
    fn fold(self, success: fn(A, usize, bool) -> B, reject: fn(usize, bool) -> B) -> B;
}

pub trait FMapResponse<A, B> {
    fn fmap(self, f: fn(A) -> B) -> Response<B>;
}

// -------------------------------------------------------------------------------------------------

impl<A, B> FoldResponse<A, B> for Response<A> {
    fn fold_fn(self, success: &Fn(A, usize, bool) -> B, reject: &Fn(usize, bool) -> B) -> B {
        match self.v {
            Some(a) => success(a, self.o, self.c),
            _ => reject(self.o, self.c),
        }
    }

    fn fold(self, success: fn(A, usize, bool) -> B, reject: fn(usize, bool) -> B) -> B {
        self.fold_fn(&success, &reject)
    }
}

impl<A, B> FMapResponse<A, B> for Response<A> {
    fn fmap(self, f: fn(A) -> B) -> Response<B> {
        match self.v {
            Some(a) => response(Some(f(a)), self.o, self.c),
            None => response(None, self.o, self.c),
        }
    }
}
