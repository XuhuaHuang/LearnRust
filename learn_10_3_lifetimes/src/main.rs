/** 10_3_lifetimes.rs
*
* https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
* every reference in Rust has a lifetime, the scope for which that reference is valid
* demo for common ways programmers might encounter lifetime syntax
* TODO: automated tests in Rust https://doc.rust-lang.org/book/ch11-00-testing.html
*
* Xuhua Huang
* Created: Aug 2, 2021
* Last updated: Aug 3, 2021
 */

mod lib;

use learn_10_3_lifetimes::{
    first_word,
    longest_with_an_announcement,
};

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
    // println!("The longest string is \"{}\"", result); // ERROR! borrowed value does not live long

    demonstrate_lifetime_structs();
    demonstrate_lifetime_strings();

    /* Static Lifetime Keyword */
    let s:&'static str = "I have a static lifetime.";
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

/* Lifetime Annotations in Structs */
// an instance of ImportantExcerpt can’t outlive the reference it holds in its part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// adding lifetime annotations in method definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> u8 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please {}", announcement);
        self.part
    }
}

fn demonstrate_lifetime_structs() {
    let novel: String = String::from("Call me Ismael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i: ImportantExcerpt = ImportantExcerpt {
        part: first_sentence,
    };
}

fn demonstrate_lifetime_strings() {
    let my_string: String = String::from("hello world");

    // first_word works on slices of `String`s
    let word: &str = first_word(&my_string[..]);

    let my_string_literal: &str = "hello world";

    // first_word works on slices of string literals
    let word: &str = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word: &str = first_word(my_string_literal);
}
