use parsers::core::Executable;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub struct Or<E>(pub E, pub E);

pub struct And<E, R>(pub E, pub R);

// -------------------------------------------------------------------------------------------------

impl<A, E> Executable<A> for Or<E> where E: Executable<A>
{
    fn execute(&self, s: &str, o: usize) -> Response<A> {
        let Or(p1, p2) = self;

        match p1.execute(s, o) {
            Response::Success(a1, i1, b1) => Response::Success(a1, i1, b1),
            Response::Reject(_, b1) => {
                match p2.execute(s, o) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(i2, b2) => Response::Reject(i2, b1 || b2)
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, B, E1, E2> Executable<(A,B)> for And<E1, E2>
    where E1: Executable<A>,  E2: Executable<B>
{
    fn execute(&self, s: &str, o: usize) -> Response<(A,B)> {
        let And(p1, p2) = self;


        match p1.execute(s, o) {
            Response::Success(a1, i1, b1) => {
                match p2.execute(s, i1) {
                    Response::Success(a2, i2, b2) => Response::Success((a1, a2), i2, b1 || b2),
                    Response::Reject(i2, b2) => Response::Reject(i2, b1 || b2),
                }
            }
            Response::Reject(i1, b1) => Response::Reject(i1, b1)
        }
    }
}

// -------------------------------------------------------------------------------------------------
