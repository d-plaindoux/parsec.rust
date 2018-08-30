extern crate parsecute;

use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::monadic::*;
use parsecute::parsers::response::*;

fn main() {
    let p = any().then(any()).fmap(Box::new(|_| 1));

    match p.execute(&"ab".as_bytes(), 0) {
        Response { v: Some(1), o: 2, c: true } => println!("All done"),
        _ => println!("Ouch"),
    }
}