// _002_hello_user.rs
// brief: print greeting to user when running with command line argument.
// to compile and run, execute:
// rustc _002_hello_user.rs
// ./_002_hello_user

use std::env;

fn main() {
    let name = std::env::args.skip(1).next(); // returns an `Optional`
    // use match statement and operand to seek for a value
    match name {
        Some(n) => println!("Hi there! {}", n),
        None => panic!("Did not receive any name!")
    }
}