/**
 * Project: mini globally search a regular expression and print (grep) program
 * 
 * To execute the project:
 * $ cd .\minigrep\
 * $ cargo build
 * $ cargo run -- test sample.txt
 * Running `target\debug\minigrep.exe test sample.txt`
 * 
 * October 24, 2022
 */


use std::env;

fn main() {

    // use function std::env::args() to read the arguments parsed to main
    // similar to int main(int argc, char** argv) in C++
    // .collect() returns the iterator to such produced series
    let args: Vec<String> = env::args().collect();
    // print the result with the "debug" macro
    // dbg!(args);
    // this is commented-out because it moves the args variable and forces its life span to end
    // thus it causes the following assignment result in errors

    /**
     * NOTE: the default argument (a.k.a argv[0]) is the path of the executable/binary
     * with respect to the root of the project: "target\\debug\\minigrep.exe"
     * to add argument, follow the example below:
     * $ cargo run -- arg1 arg2
     */

    /* saving the argument values in variables */
    // args[0] is `target\debug\minigrep.exe`
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Searching for {:#?}", query);
    println!("In file {:#?}", file_path);
}
