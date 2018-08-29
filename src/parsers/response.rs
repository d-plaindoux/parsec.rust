pub struct Response<A>(pub Option<A>, pub usize, pub bool);

#[inline]
pub fn response<A>(v: Option<A>, o: usize, c: bool) -> Response<A> {
    Response(v, o, c)
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
        let Response(v, s, b) = self;

        match v {
            Some(a) => success(a, s, b),
            _ => reject(s, b)
        }
    }

    fn fold(self, success: fn(A, usize, bool) -> B, reject: fn(usize, bool) -> B) -> B {
        self.fold_fn(&success, &reject)
    }
}

impl<A, B> FMapResponse<A, B> for Response<A> {
    fn fmap(self, f: fn(A) -> B) -> Response<B> {
        let Response(v, s, b) = self;

        match v {
            Some(a) => response(Some(f(a)), s, b),
            None => response(None, s, b)
        }
    }
}