/** Structure in Rust
  * Xuhua Huang
  * Feb 2021
  */

// compare contrast tuples and structs
// object-oriented concept
// data attributes, methods and functions

// Define and instantiating structure
// key: value pairs
// consider a structure as a template/blueprint
// instances fill in the template with particular values
struct User {
    // fields
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs Color and Point
struct Color(u8, u8, u8); // unsigned 8-bit integer for RGB
struct Point(i32, i32, i32); // signed integer for 3-axis coordinates

fn main() {
    println!("Hello, world!");
    println!("Let's talk about structure in Rust!");

    println!("\nA structure \"User\" is defined");
    println!("Instantiating an object");

    // not necessary to specify the fields in the same order declared
    // user1 is immutable without the keyword "mut"
    let mut user1: User = User {
        user_name: String::from("user1"),
        email: String::from("one@email.com"),
        sign_in_count: 1,
        active: true,
    }; // end user1

    println!("\nChanging email field of user1");
    println!("Email address is: \"{}\" before changing", user1.email);
    user1.email = String::from("xuhua.huang.io@outlook.com");
    println!("After changing, email of user1 is now: \"{}\"", user1.email);

    // Struct Update Syntax
    // creating instances from other instances with struct update syntax
    let user2: User = User {
        user_name: String::from("user2"),
        email: String::from("another@email.com"),
        ..user1 // sign_in_count: user1.sign_in_count,
                // active: user1.active,
        // ".." syntax specifies the remaining field have the same value as the given instance
    };

    // using tuple structs without named fields to create different types
    // tuple struct ignores the name of each field because they are verbose and redundant
    let black: Color = Color(0, 0, 0);     // Color tuple struct
    let origin: Point = Point(0, 0, 0);    // Point tuple struct

    // unit-like structs without any field behave like signature of variables as its name suggests
}

// function returns a User type structure
fn build_user(arg_email: String, arg_user_name: String) -> User {
    User {
        email: arg_email,
        user_name: arg_user_name,
        active: true,
        sign_in_count: 1,
    } // this user is instantiated and returned as an expression without ending semicolon
}
