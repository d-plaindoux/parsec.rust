extern crate parsecute;

use parsecute::parsers::core::*;
use parsecute::parsers::response::*;
use parsecute::parsers::basic::*;
use parsecute::parsers::flow::*;

fn main() {
    let p = And(Any(), Any());

    match p.execute("ab", 0) {
        Response::Reject(_, _) => println!("Ouch"),
        Response::Success(_, _, _) => println!("All done"),
    }
}