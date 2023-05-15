fn invoke_closure<T>(a: i32, b: i32, function: T) -> i32
where T: Fn(i32, i32) -> i32 {
    function(a, b)
}

fn main() {
    let sum = |a: i32, b: i32| -> i32 { a + b };
    let prod = |a: i32, b: i32| -> i32 { a * b };

    println!("4 + 5 = {}", invoke_closure(4, 5, sum));
    println!("4 * 5 = {}", invoke_closure(4, 5, prod));
}
