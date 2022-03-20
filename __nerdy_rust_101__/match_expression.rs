/**
 * match_expression.rs
 * brief: filter-like keyword `match` and `=>` operand in Rust; to compile and run, execute:
 * $ rustc match_expression.rs
 * $ ./match_expression
 */

fn request_status() -> u32 {
    200
}

fn main() {
    let status: u32 = request_status();

    /* Use a match expression to determine the status */
    match status {
        200 => println!("Request Success"),
        404 => println!("Not Found"),
        other => {
            println!("Request failed with code: {}", other);
        }
        // other is equivalent to using _ to ignore irrelevant values
    }
}