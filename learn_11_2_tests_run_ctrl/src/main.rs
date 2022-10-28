fn main() {
    println!("Hello, world!");
    let mut greetings: String = "Greetings".to_lowercase().to_string();

    /*! using built-in debug macro */
    dbg!(greetings);
    // greetings is moved when using debug macro
    // no longer valid
    greetings = "Greetings".to_uppercase().to_string();
    dbg!(greetings);
}
