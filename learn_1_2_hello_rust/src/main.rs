extern crate ferris_says;
use ferris_says::say; // added to project dependencies
use std::io::{stdout, BufWriter, Stdout}; // nested use statement
// equivalent to std::io::stdout, std::io::BufWriter

fn main() {
    // instantiate an output stream object with default constructor stdout()
    let stdout: Stdout = stdout();

    // create a string named message
    // assign the literal using String::from() function
    // If we were in C++: namespace String { from(const char* str){} }
    let message: String = String::from("Hello Rustaceans!");

    // convert to character array and count elements
    let width: usize = message.chars().count();

    // pass stdout.lock() as parameter to constructor
    let mut writer: BufWriter<std::io::StdoutLock> = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
