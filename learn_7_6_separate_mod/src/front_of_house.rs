/** front_of_house.rs
  * demonstrate separating modules into different files
  *
  * Xuhua Huang
  * April 2021
  */

pub mod hosting {
    pub fn get_empty_space() -> bool {
        println!("front_of_house::[pub]hosting::[pub]fn get_empty_space");
        println!("If there is an empty space, return true; if not, return false");
        true
    }

    pub fn add_to_waitlist() {
        println!("front_of_house::[pub]hosting::[pub]fn add_to_waitlist");
    }

    pub fn seat_at_table() {
            println!("front_of_house::[pub]hosting::[pub]fn seat_at_table");
    }
}