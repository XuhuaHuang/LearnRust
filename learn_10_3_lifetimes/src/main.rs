/** 10_3_lifetimes.rs
*
* https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
* every reference in Rust has a lifetime, the scope for which that reference is valid
* demo for common ways programmers might encounter lifetime syntax
*
* Xuhua Huang
* Created: Aug 2, 2021
* Last updated: Aug 2, 2021
 */

fn main() {
    println!(); // added this python style of new line
    println!("Let's talk about variable lifetimes in Rust!");

    /* Prevent Danging References with Lifetimes */
    // firstly, let's start with a customized scope
    {
        // let r; // move this line to line 22 to prevent compiler error
        {
            let x: u8 = 5;
            let r: &u8 = &x; // ERROR! borrowed value does not live long enough
            println!("r: {}", r);
        }
        // println!("r: {}", r);
    }

    /* Generic Lifetimes in functions */
    let alphabet_head: String = String::from("abcd");
    let alphabet_tail: String = String::from("xyz");
    // call functions that parsed by reference
    let result = find_longest_str(alphabet_head.as_str(), alphabet_tail.as_str());
    println!("The longest string is {}", result);

    /* Lifetime Annotation Syntax */
    // &i32 - a reference to 32-bit signed integer
    // &'a i32 - a reference with an explicit lifetime
    // &'a mut i32 - a mutable reference with an explicit lifetime
    let long_str: String = String::from("This is a long string");
    let result;
    // start a custom scope
    {
        let alphabet_tail: String = String::from("xyz");
        result = find_longest_str(long_str.as_str(), alphabet_tail.as_str());
    }
    println!("The longest string is \"{}\"", result);
}

// Previously method signature: fn find_longest_str(x: &str, y: &str) -> &str {
// in this case, the return type shares the same lifetime as parsed-in arguments
fn find_longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}