/**
 * if_else.rs
 * brief: explore conditional statment in Rust; to compile and run, execute:
 * $ rustc if_else.rs
 * $ ./if_else
 */

 /**
  * About conditions in Rust
  * if-else keywords in Rust aren't neccessarily statements but rather an expression
  * statements do not return, expressions do; and if-else in Rust always returns SOME value
  */

fn main() {
/* Simple demonstration */
let rust_is_awesome: bool = true;
// following if-else returns empty () as the actual value
if rust_is_awesome {
    println!("Indeed Rust is awesome!");
} else {
    println!("Well, you should give it a try!");
}

/* If and variable assignment */
let result: &str = if 1 == 2 {
    "Wait, hold on, this doesn't make sense."
} else {
    "Rust actually makes sense." // this value will be assigned
};
println!("Result of the comparison: {:#?}", result);
}