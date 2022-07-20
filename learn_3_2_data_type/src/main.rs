/** Data Types in Rust
  * two data type subsets
  * scalar and compound
  *
  * Xuhua Huang
  * Feb 2021
  */

fn main() {
    #[allow(unused_variables)]

    // INTEGER
    // could be signed or unsigned
    // signed integer types start with "i", unsigned integer types start with "u"
    // length could be 8-bit, 16-bit, 32-bit, 64-bit, 128-bit
    // notice that signed numbers are stored using two's complement representation
    // relying on integer overflow's wrapping behavior is considered as an error

    // unsigned 8-bit integer
    // will overflow outside of [0,255] inclusive
    let int1: u8 = 255;
    println!("The value of an unsigned 8-bit integer \"int1\" is: {}", int1);

    // FLOATING-POINT
    // f32 and f64 -> 32-bit and 64-bit respectively
    let x = 2.0; // defaulted to f64 with double precision
    // let x: f64 = 2.0;
    let y: f32 = 3.0; // f32 with single precision
    // output
    println!("The value of a 64-bit floating point \"x\" is: {}", x); // 2.0
    println!("The value of a 32-bit floating point \"y\" is: {}", y); // 3.0

    // NUMERIC OPERATIONS
    let _sum = 5 + 10; // addition
    let _difference = 95.5 - 4.3; // subtraction
    let _product = 4 * 30; // multiplication
    let _quotient = 56.7 / 32.2; // division
    let _remainder = 43 % 5; //

    // BOOLEAN
    let _t: bool = true;
    let mut _f: bool = false; // with explicit type annotation
    _f = true; // success with keyword "mut", as discussed in the previous notes

    // CHARACTER
    // 4 bytes Unicode Scalar Value
    // ranged [U+0000, U+D7FF] and [U+E000 , U+10FFFF]
    let _c: char = 'c';
    let _first_name: &str = "Xuhua";
    let _last_name: String = String::from("Huang");

    // COMPOUND
    // group multiple values into one type
    // 2 primitive compound types -> tuples and arrays
    // tuple has a fixed length, can not grow or shrink once declared
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup is a tuple with 32-bit signed integer, 64-bit floating point, 8-bit unsigned integer
    let (x, y, z) = tup; // tie three variables to tuple tup, automatically linked
    // output
    println!("\nPrinting tuple \"tup\" elements x, y, z sequentially: {}, {}, {}", x, y, z);

    // access tuple element with "tuple_name.index" -> index the value want to access
    let five_hundred: i32 = tup.0; // 600
    let six_point_four: f64 = tup.1; // 6.4
    let one: u8 = tup.2;
    // output
    println!("\nAccessing tuple \"tup\" elements with \"tuple_name.index\": {}, {}, {}",
        five_hundred, six_point_four, one
    );

    // ARRAY
    // element in the array must have the same type with fixed size
    // written as comma-separated list inside square brackets - [1, 2, 3]
    let _months: [&str; 12] = [
        "January", "February", "March",
        "April", "May", "June",
        "July", "August", "September",
        "October", "November", "December"
    ];

    // let array_name: [data_type; array_size] = [value, value, ...];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // array_name: a
    // data_type: 32-bit signed integer
    // array_size: 5

    // let array_name = [initial_value; size];
    // array of size provided with initial_value as elements
    let _a: [u8; 5] = [3; 5]; // [3, 3, 3, 3, 3]

    // access individual element
    // array_name[index_of_element]
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // reuse the same variable name as discussed before
    let _first_element: i32 = _a[0];
    let _second_element: i32 = _a[1];

    // INVALID ARRAY ELEMENT ACCESS
    // et tenth_element = a[10]; // panic at run time -> index out of bounds
    // println!("The tenth element is: {}", tenth_element); // ERROR!

    // UPCOMING - functions
}
