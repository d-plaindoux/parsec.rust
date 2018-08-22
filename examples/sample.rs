extern crate parsecute;

use parsecute::parsers::core::*;
use parsecute::parsers::response::*;
use parsecute::parsers::basic::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::monadic::*;

fn main() {
    let p = any().then(any()).fmap(Box::new(|_| 1));

    match p.execute("ab", 0) {
        Response::Reject(_, _) => println!("Ouch"),
        Response::Success(_, _, _) => println!("All done"),
    }
}