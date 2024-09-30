/**
 * loops.rs
 * brief: exploring while loops and for loops with Range in Rust
 * $ rustc loops.rs
 * $ ./loops
 */

fn main() {
    let mut x: i64 = 1024;

    /* Launch an infinite loop with manual breaking out */
    loop {
        if x < 0 {
            break;
        }
        println!("{} more runs to go to reach negative range", x);
        x -= 1;
    }

    /* Equivalent while loop syntax */
    let mut x = 1024; // variable shadowing
    while x > 0 {
        println!("{} more runs to go to reach negative range", x);
        x -= 1;
    }

    // for loop in Rust
    // for keyword is more like an iterator-like syntax sugar
    // typical usage is with a Range
    // print number 1 to 10 (inclusive) with a for loop
    // not having the = operator results an noninclusive range
    for i in 0..=10 {
        println!("{}", i);
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }

    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");
}
