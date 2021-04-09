/** 8_2_utf8_strings.rs
  * UTF-8 Encoded Strings
  *
  * Xuhua Huang
  * April 2021
  */

/** 3 major headache:
  * Rust’s propensity for exposing possible errors
  * strings being a more complicated data structure than many programmers give them credit for
  * UTF-8
  *
  * operations on String that every collection type has, such as creating, updating, and reading
  * also discuss the ways in which String is different from the other collections
  * String: growable, mutable, owned, and UTF-8 encoded
  * Rust also includes other string types: OsString, IsStr, CString and CStr
  */

fn main() {
    println!("Let's about UTF-8 Encoded Strings");

    /* CREATE A NEW STRING */
    let mut str = String::new(); // create a new, empty string
    let data = "initial contents";
    str = data.to_string();
    print_string(&str);

    // the method to_string() also works on a literal directly
    // use to_string() method to create a String object from a string literal
    str = "initial contents with to_string() method".to_string();
    print_string(&str);

    // EQUIVALENT: String::from("string literals")
    str = String::from("initial contents with String::from() method");
    print_string(&str);

    /* UTF-8 ENCODED DATA */
    let hello_mandarin = String::from("你好");
    print_string(&hello_mandarin);

    /* UPDATING A STRING */
    // grow in size and its content can change
    // + operator and format! marco to concatenate strings

    /* Append to strings with push_str and push */
    str = String::from("Waiting for appending.");
    str.push_str(" Appended to string named \"str\"");
    print_string(&str);

    let mut original_string = String::from("original string");
    let to_append: &str = " push_str() with a String object";
    original_string.push_str(to_append); // appended to String object
    print_string(&original_string);

    /* The push method takes a single character as a parameter and adds it to the String */
    let mut s = String::from("lo");
    s.push('l');
    println!("String object named \"s\" contains content \"lol\"");
    print_string(&s);

    /* Concatenation with + operator or format! marco */
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let hello_world = hello + &world;
    // added a reference of String world to the String hello
    // implicit conversion from &String to &str to fit + operator
    // hello has been moved and no longer available
    println!("\nTwo String objects are concatenated to form Hello, world!");
    print_string(&hello_world);

    /* format! marco */
}

// function to print the content of a String object
// with immutable reference
fn print_string(str: &String) {
    println!("{}", str);
}