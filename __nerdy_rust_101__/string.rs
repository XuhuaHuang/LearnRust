// string.rs
// brief: explore string (both string literals &str and String objects) in Rust
// to compile and run, execute:
// rustc string.rs
// ./string

fn main() {
    let greeting_question: &str = "How are you doing?"; // &str type; a pointer to a character pointer.
    // &str objects are allocated on the stack.
    // `String` objects are allocated on the heap.
    let person: String = "Andy".to_string();

    println!("Hello {}\n{}", person, greeting_question);
}