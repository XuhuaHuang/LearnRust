/**
 * static_variables.rs
 * brief: explore static variables in Rust
 * to compile and run, execute:
 * rustc static_variables.rs
 * ./static_variables
 */

 /**
  * global variables must be initialized when declared.
  * initializations of global variables must be compile-time constant
  * keyword "unsafe" must be used along with usage of mutable global variables.
  */

#[feature(const_fn)]
fn main() {
    /* local variables declaration and initialization */
    let x;          // declare local variable x
    let y = 1_i32;  // declare and initialize local variable y
    x = 2_i32;      // initialize local variable x
    println!("x = {}, y = {}", x, y);

    /* static variables */
    static PI: f64 = 3.14159265359;
    println!("PI = {}", PI);


    /* mutable global variables and reassignment */
    static mut G: f64 = 9.9;
    unsafe {
        println!("G = {}", G);
    }
    unsafe {
        G = 9.81;
        println!("G = {}", G);
    }

    /* compile time constant function - constant expression in C++ */
    use std::sync::atomic::AtomicBool;
    static FLAG: AtomicBool = AtomicBool::new(true);
    println!("FLAG = {:#?}", FLAG);
}
