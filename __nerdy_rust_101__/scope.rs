/**
 * Rust has a concept of ownership with a set of rules that the compiler checks at compile time.
 * rustc scope.rs
 * ./scope
 */

fn main() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");

    // Shadowing is different from mutation, because after shadowing both variables' memory locations exist at the same time.
    // Both are available under the same name, depending where you use it in the code.
    //! A shadowing variable can have a different type.
    //! Shadowing looks obscure at first, but is convenient for holding on to values after .unwrap().
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}
