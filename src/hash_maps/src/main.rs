use std::collections::HashMap;

fn main() {
    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    println!("Length of heroes hash map: {}", heroes.len());

    if heroes.contains_key(&"Batman") {
        let the_batman: Option<&&str> = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("{} is a hero", x),
            None => println!("Heroes hash map does not contain Batman"),
        }
    }
}
