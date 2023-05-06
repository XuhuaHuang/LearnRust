#![allow(unused)]

/** 9_2_recover_error.rs
 * Recoverable errors with Result enum
 *
 * Xuhua Huang
 * May 2021
 */

/* Recall from earlier: type 'Result'
 * enum Result<T,E> {
 *     Ok(T),
 *     Err(E),
 * }
 */
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("Let's talk about recoverable errors in Rust!");

    // attempt to open an nonexisting file
    let file: Result<File, io::Error> = File::open("hello.txt"); // this line compiles
    // let f:u32 = File::open("hello.txt"); // this line gives a type error

    /* About File::open(path: "") */
    // Return type: Result<T, Error>
    // T - generic success value, std::fs::File, file handler
    // E - error value, std::io::Error

    /* Use a 'match' expression to handle 'Result' variants */
    /*
    let f = match file {
        Ok(file) => file,
        Err(error) => panic!("With simple 'match' statement, " +
                             "problem opening the file: {:#?}", error),
    };
    */
    // commented out since file borrowed after

    /* Matching on Different Errors */
    let f: Result<File, io::Error> = match file {
        Ok(file) => Ok(file), // return file handle
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => Ok(fc),
                Err(e) => panic!("Problem creating the file: {:#?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:#?}", other_error)
            }
        }, // end handling error
    }; // end matching

    let mut file_content: String = String::new();

    let _ = match f
        .expect("Failed to read from file hello.txt")
        .read_to_string(&mut file_content)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    println!("Content from hello.txt: {:#?}", file_content);

    /* Shortcuts for Panic on Error: unwrap and expect */
    let _f: File = File::open("hello.txt").unwrap();
    let _f: File = File::open("hello.txt").expect("Failed to open hello.txt");
    // expect() takes parameter as customized error message

    /* Propagating Errors */
    // see function definition read_username_short()
}

// return type: Result<T, E>, generic parameter T and E
// T has been filled with type String
// E has been filled with type io::Error
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result: Result<File, io::Error> = File::open("username.txt");

    let username_file: Result<File, io::Error> = match username_file_result {
        Ok(file) => Ok(file),
        Err(e) => return Err(e),
    };

    let mut username: String = String::new();

    // call 'read_to_string()' method on the file handle
    // read content into String 's' Hint: mutable reference/pointer
    match username_file?.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/* ? operator */
// error and values that have ? operator called on will go through 'from' function
// in the 'From' trait in the standard library
// returns the value inside an Ok to the calling variable
// if an error occurs, ? operator returns early and give any occur Err
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file: File = File::open("username.txt")?;
    let mut username: String = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // equivalent to:
    // File::open("username.txt")?.read_to_string(&mut username)?;
}
