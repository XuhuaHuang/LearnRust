/** 10_1_generic_data_types.rs
 * Define functions, structs, enums and methods using generics
 * Discuss how generics affect code performance
 * `Template` in C++
 *
 * Rust compiles generic code into code that specifies the type in each instance
 * programmers pay no runtime cost for using generics -> extremely efficient at runtime
 *
 * Xuhua Huang
 * Created: July 24, 2021
 * Last updated: July 31, 2021
 */

/**
 * function: largest_i32
 * param: a list of 32-bit integer, borrowed by reference
 * return type: 32-bit integer
 */
fn largest_i32(list: &[i32]) -> i32 {
    // assuming the first one is the largest number
    let mut largest: i32 = list[0];
    // loop through the list and compare to the original item
    // then decide whether to replace the variable
    for &item in list {
        if item > largest {
            largest = item;
        } // end if
    } // end for
    largest
}

/**
 * function: largest_char
 * param: a list of characters, borrowed by reference
 * return type: character
 */
fn largest_char(list: &[char]) -> char {
    // assuming the first one is the largest character
    let mut largest: char = list[0];
    // loop through the list and compare to the original item
    // then decide whether to replace the variable
    for &item in list {
        if item > largest {
            largest = item;
        } // end if
    } // end for
    largest
}

/**
 * Function that uses  generic typing with parameter T
 * does not compile as of now, commented out line 51 - 59
 */
fn largest<T>(list: &[T]) -> T
where
    T: std::cmp::PartialOrd<T> + Clone,
{
    let mut largest: T = list[0].clone();
    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

/* Generic typing in public struct */
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

/* Generic typing in Enum definitions */
// Option<T> can express the abstract concept of having an optional value
enum Result<T, E> {
    Ok(T),
    // operation succeeded, return a value of SOME type
    Err(E),
    // operation failed, return an error of SOME type E
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// Point<f32> instances rather than on Point<T> instances with any generic type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }
}

fn main() {
    println!(); // added this python style of new line
    println!("Let's talk about generic data types in Rust!");

    /* Generic data types used in function definitions */
    // placing the generics in the signature of the function
    // makes code more flexible and provides functionality to callers

    // using function 'largest_i32(list: &[32])'
    let num_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let result: i32 = largest_i32(&num_list);
    println!("The largest number in the list is: {}", result);

    // using function 'largest_i32(list: &[char])'
    let char_list: Vec<char> = vec!['y', 'm', 'c', 'a'];
    let result: char = largest_char(&char_list);
    println!("The largest character in the list is: {}", result);

    // using public struct Point
    let integer: Point<i32> = Point { x: 5, y: 10 };
    let float: Point<f64> = Point { x: 1.0, y: 4.0 };
    println!("Point created with integer is {:#?}", integer);
    println!("Point created with float is {:#?}", float);
    // *note: just like in C++, x and y have to share the same type 'T' unless otherwise specified
    // let point_wont_work = Point {x: 1, y: 5.0}; // expected integer, found floating-point number
    // for example, struct Point<Int, Flt> { x: Int, y: Flt, }

    // using struct `Point` implementation block
    let point_demo: Point<i32> = Point { x: 5, y: 10 };
    println!(
        "\nPrinting struct `Point` object `point_demo`
    point_demo.x = {}
    point_demo.y = {}",
        point_demo.x(),
        point_demo.y()
    );

    let point_f32: Point<f32> = Point { x: 5.5, y: 11.0 };
    println!(
        "\nPrinting distance from origin for `point_32`
    distance from origin: {}",
        point_f32.distance_from_origin()
    );
}
