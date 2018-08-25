extern crate parsecute;

use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::flow::*;
use parsecute::parsers::response::*;

fn main() {
    let p = any().rep();

    let s = "a".repeat(1024 * 1024 * 500); // 500M

    println!("Start parsing ...");

    match p.execute(s.as_bytes(), 0) {
        Response::Success(_, o, _) => println!("All done {}", o),
        _ => println!("Ouch"),
    }
}