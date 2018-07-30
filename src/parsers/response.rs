pub enum Response<A> {
    Success(A, String, bool),
    Reject(bool),
}

type OnSuccess<A, B> = fn(A, String, bool) -> B;
type OnReject<B> = fn(bool) -> B;

pub trait Fold<A, B> {
    fn fold(self, success: OnSuccess<A, B>, reject: OnReject<B>) -> B;
}

impl<A, B> Fold<A, B> for Response<A> {
    fn fold(self, success: OnSuccess<A, B>, reject: OnReject<B>) -> B {
        match self {
            Response::Success(a, s, b) => success(a, s, b),
            Response::Reject(b) => reject(b)
        }
    }
}
