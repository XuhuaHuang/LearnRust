/** 7_5_keyword_use.rs
  * bring mods and traits into scope
  * use, pub use keyword
  *
  * Xuhua Huang
  * April 2021
  */

use std::collections::HashMap;

// use std::cmp::Ordering;
// use std::io;
// ->
use std::{cmp::Ordering, io};

// use std::io::prelude::*;
// use std::io::Write;
// use std::io::BufReader
// ->
use std::io::{prelude::*, Write, BufReader};

// globe operator *
// bring all public items in trait "collections"
use std::collections::*;

fn main() {
    println!("Let's talk about the keyword \"use\" in Rust!");

    /* create a new mutable hash map and name it "map" */
    let mut map: HashMap<u8, u8> = HashMap::new();
    map.insert(1, 2);
}
