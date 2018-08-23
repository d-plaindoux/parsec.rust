extern crate parsecute;

use parsecute::parsers::execution::*;
use parsecute::parsers::response::*;
use parsecute::parsers::basic::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::monadic::*;

fn main() {
    let p = any().then(any()).fmap(Box::new(|_| 1));

    match p.execute("ab", 0) {
        Response::Success(1, 2, true) => println!("All done"),
        _ => println!("Ouch"),
    }
}