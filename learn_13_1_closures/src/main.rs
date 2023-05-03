use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

/// Inventory structure implementation block.
///
/// # Examples
///
/// ```
/// let store = Inventory {
///     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
/// };
/// let user_pref1 = Some(ShirtColor::Red);
/// let gave_away: ShirtColor = store.giveaway(user_pref1);
/// ```
///
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red: u32 = 0;
        let mut num_blue: u32 = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store: Inventory = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1: Option<ShirtColor> = Some(ShirtColor::Red);
    let giveaway1: ShirtColor = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2: Option<ShirtColor> = None;
    let giveaway2: ShirtColor = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(1u32);

    /* capturing references or moving ownership */
    // https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-references-or-moving-ownership
    // A closure body can do any of the following:
    // move a captured value out of the closure, mutate the captured value, neither move nor mutate the value
    // or capture nothing from the environment to begin with.
    let list: Vec<u8> = vec![1, 2, 3];
    println!("Before defining only_borrows closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling only_borrows closure: {:?}", list);
    only_borrows();
    println!("After calling only_borrows closure: {:?}", list);

    // using variable shadowing to create a mutable variant of the list in question
    let mut list: Vec<u8> = vec![4, 5, 6];
    println!("Before defining borrows_mutably closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling borrows_mutably closure: {:?}", list);

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
