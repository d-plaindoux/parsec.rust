pub enum Response<A> {
    Success(A, usize, bool),
    Reject(usize, bool),
}

pub trait Fold<A, B> {
    fn fold_fn(self, success: &Fn(A, usize, bool) -> B, reject: &Fn(usize, bool) -> B) -> B;
    fn fold(self, success: fn(A, usize, bool) -> B, reject: fn(usize, bool) -> B) -> B;
}

impl<A, B> Fold<A, B> for Response<A> {
    fn fold_fn(self, success: &Fn(A, usize, bool) -> B, reject: &Fn(usize, bool) -> B) -> B {
        match self {
            Response::Success(a, s, b) => success(a, s, b),
            Response::Reject(s, b) => reject(s, b)
        }
    }

    fn fold(self, success: fn(A, usize, bool) -> B, reject: fn(usize, bool) -> B) -> B {
        self.fold_fn(&success, &reject)
    }
}
