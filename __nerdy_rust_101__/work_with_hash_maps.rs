// work_with_hash_maps.rs
// brief: exploring how to create a hash map, modify key-value pairs in Rust
// to compile and run, execute:
// rustc work_with_hash_maps.rs -o hash_maps.exe
// ./hash_maps

fn main() {
    use std::collections::HashMap;
    // using the factory function new() to create a new HashMap
    // specify the type name with HashMap<String, String> syntax
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    // Read a key value from the hash map
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));
    
    // Remove a key-value pair from the hash map
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}
