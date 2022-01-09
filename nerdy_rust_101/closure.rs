// closure.rs
// brief: explore closure (similar to lambda) in Rust
// to compile and run, execute:
// rustc closure.rs
// ./closure

fn main() {
    let doubler = |x| x * 2; // define a callable object inside main function
    let value = 5;           // initialize some random variable

    /* Call closure with int 4 */
    let twice = doubler(value);
    println!("{} doubled is {}.", value, twice);

    /* Define a closure with 2 arguments */
    // arguments are both 32-bit signed integer
    let big_closure = |b: i32, c: i32| {
        let sum = b + c;
        sum * twice // implicit return statement; twice = 10
    };

    let big_closure_result = big_closure(1, 2); // (1+2) * 10 = 30
    println!("Result from the big closure: {}", big_closure_result);
}