/** Variable and Mutability in Rust
  * Xuhua Huang
  * March 2021
  */

fn main() {

    // create a mutable variable named "x"
    // mutable means the value of this variable can be overwritten
    // variable without the mut keyword could be understood as static
    let mut x = 5;

    // create another variable named "y" without keyword "mut"
    let y = 6;

    // attempt to change the value of both x and y
    // changing the value of an immutable y will cause an error
    x = 7;
    // y = 8; // ERROR!

    // output result
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // Shadowing
    // create variable with the same name with keyword "let"
    // the newer variable shadows the previous one
    // newer value of the variable appears when it is used
    println!("Variable \"y\" is being shadowed");

    let y = y + 1;
    println!("The value of y is: {}", y); // 7

    let y = y * 2;
    println!("The value of y is: {}", y); // 14

    // ADVANTAGE of shadowing
    // effectively creating new variable with "let" -> possible to change type
    // reuse the same variable name

    // UPCOMING: variable type
    // let mut tab = "    "; // contains 4 \s
    // tab = tab.len(); // ERROR! "expected `&str`, found `usize'"
}
