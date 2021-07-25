/** 10_1_generic_data_types.rs
 * Define functions, structs, enums and methods using generics
 * Discuss how generics affect code performance
 * `Template` in C++
 * Xuhua Huang
 * Created: July 24, 2021
 * Last updated: July 24, 2021
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
/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/

/* Generic typing in public struct */
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    println!(); // added this python style of new line
    println!("Let's talk about generic data types in Rust!");

    /* Generic data types used in function definitions */
    // placing the generics in the signature of the function
    // makes code more flexible and provides functionality to callers

    // using function 'largest_i32(list: &[32])'
    let num_list = vec![34, 50, 25, 100, 65];
    let result: i32 = largest_i32(&num_list);
    println!("The largest number in the list is: {}", result);

    // using function 'largest_i32(list: &[char])'
    let char_list = vec!['y', 'm', 'c', 'a'];
    let result: char= largest_char(&char_list);
    println!("The largest character in the list is: {}", result);

    // using public struct  Point
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
    println!("Point created with integer is {:#?}", integer);
    println!("Point created with float is {:#?}", float);
    // *note: just like in C++, x and y have to share the same type 'T' unless otherwise specified
    // let point_wont_work = Point {x: 1, y: 5.0}; // expected integer, found floating-point number
    // for example, struct Point<Int, Flt> { x: Int, y: Flt, }
}

// TODO: generics in Enum definitions