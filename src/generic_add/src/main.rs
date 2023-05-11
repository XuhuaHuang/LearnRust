use std::ops::Add;

/**
 * When we type `a: T`, we are restraining the type of `a` to be `T`
 * When we type `T: std::ops::Add<Output = T>`, we are constraining the type T
 * to support the trait std::ops::Add, and produce the output of the same type
 */
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[derive(Debug, Clone)]
struct MyInt(i32);

impl std::ops::Add for MyInt {
    type Output = MyInt;

    fn add(self, rhs: MyInt) -> MyInt {
        MyInt(self.0 + rhs.0)
    }
}

fn main() {
    println!("3 + 4 = {}", add(3, 4));

    let a = MyInt(1);
    let b = MyInt(2);
    println!("{:?}", a);
    println!("{:?}", b);

    let c = a.clone() + b.clone();
    println!("{} + {} = {}", a.0, b.0, c.0);
}
