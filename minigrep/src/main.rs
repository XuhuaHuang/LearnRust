/**
 * Project: mini globally search a regular expression and print (grep) program
 * https://github.com/env-logger-rs/env_logger/blob/main/examples/default.rs
 * 
 * To execute the project:
 * $ cd .\minigrep\
 * $ cargo build
 * $ cargo run -- test poem.txt
 * Running `target\debug\minigrep.exe test poem.txt`
 * 
 * For content on how to read from a file, check out:
 * learn_9_2_recover_error\src\main.rs
 * 
 * Xuhua Huang
 * October 24, 2022
 */


// #[macro_use]
// extern crate log;
use env_logger::Env;
use log::{debug, error, info};
// use log::{debug, error, log_enabled, info, Level};
use std::{env, fs, process};

fn main() {

    /* Initialize logger */
    let env: env_logger::Env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    // env_logger::init();

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

    // std::fs::read_to_string returns std::io::Result<String, Error>
    // if an error is thrown, msg parsed to .expect() will print to terminal
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

}
