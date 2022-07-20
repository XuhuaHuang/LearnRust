
/** References and Borrowing in Rust
  * Xuhua Huang
  * Feb 2021
  */

/** INTRODUCTION
  * Pass argument to function by references
  * instead of let function take ownership and return
  *
  * Referencing: &
  * Dereferencing: *
  * At any given time, there could be either one mutable reference or multiple immutable ones
  * References must be always valid
  */

fn main() {
    println!("Hello, world!");
    println!("Let's talk about references in Rust!\n");

    let greeting: String = String::from("Hello");
    let length_greeting: usize = calculate_length(&greeting);
    println!("The length of string \"Hello\" is {}", length_greeting);
    // &greeting creates a reference to the value of string without taking the ownership
    // will not be dropped after the execution of function

    // MUTABLE REFERENCES
    let mut greeting = String::from("Hello");
    change_string(&mut greeting);
    println!("The greeting string is now: {}", greeting);

    // NOTE: only one mutable reference to a particular variable
    // Multiple references is allowed, as long as there isn't a mutable reference in the same scope
    let ptr1 = &mut greeting; // OK, valid mutable borrow within the main() scope
    // let ptr2 = &mut greeting; // ERROR! Pointer is already captured with ptr1
    // println!("{}, {}", ptr1, ptr2); // ERROR!
    println!("\nWith mutable reference ptr1: {}", ptr1);
    // ptr1 is no longer valid here

    let ref1 = &greeting; // OK
    let ref2 = &greeting; // OK
    println!("\nWith two references ref1 and ref2: {} and {}", ref1, ref2);
    // ref1 and ref2 are no longer valid here

    // DANGLING REFERENCES
    // a pointer that points to a memory that contains nothing
    // compiler in Rust ensures reference will not be dangling
    // the data will not go out of the scope before the reference does
    let _dangling_ptr: &String = dangle_pointer();

    // UPCOMING: slices
}

// function that asks for a reference without taking over the ownership
// we call having references as function parameters borrowing
// data passed by reference cannot be **borrowed** as mutable without "mut"
fn calculate_length(arg_str: &String) -> usize {
    arg_str.len() // using expression to return
}

// function that asks for a mutable reference
fn change_string(arg_str: &mut String) {
    arg_str.push_str(", world!");
}

// attempting to return a dangling pointer
// ERROR! no lifetime specifier
// this function returns a borrowed value, but there is nothing to borrow from
fn dangle_pointer() -> &'static String  { // attempting to return a reference to a String
    let str = String::from("Hello from \"dangle_pointer\"");
    &str // returning a reference to "str"
}        // "str" goes out of the scope and dropped, whose memory goes away
