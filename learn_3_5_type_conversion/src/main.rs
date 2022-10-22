/**
 * main.rs
 * data type conversion in Rust
 * Xuhua Huang (xuhua.huang.io@gmail.com)
 * October 08, 2022
 * 
 * cd .\learn_3_5_type_conversion
 * cargo build
 * cargo run
 */

fn main() {
    println!("Let's talk about data type conversion in Rust!");

    let index: u8 = 1;
    let converted_up_dx: i32 = index as i32;
    println!("index = {:#?}, converted_up_idx = {:#?}", index, converted_up_dx);

    /* constraint for using the "as" syntax conversion */
    let hello_world: String = "Hello, world".to_string();
    println!("{:#?}", hello_world);
    // let hello_world_u32 = hello_world as u32; // compiler error

    /* chained conversion from &i32 to * mut i32 - borrowed immutable reference to mutable pointer */
    let index: i32 = 42; // shadowing previous declared variable under the same name
    let mutable_ptr_idx: *mut i32 = index as *const i32 as *mut i32;
    println!("mutable_ptr_idx = {:#?}", mutable_ptr_idx);
}
