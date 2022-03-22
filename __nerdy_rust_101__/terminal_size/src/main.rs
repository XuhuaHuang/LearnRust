/**
 * terminal_size.rs
 * brief: use term_size crate to determine the size of the output terminal
 * 
 * $ cd ./__nerdy_rust_101__/terminal_size
 * $ cargo build
 * $ cargo run
 */

fn main() {
    if let Some((width, height)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", width, height);
    } else {
        println!("Unable to get terminal size :(")
    }
}