use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of age {}", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Xuhua Huang"),
        age: 21,
    };
    println!("Person: {}", person);
    println!("Person: {:#?}", person);
}
