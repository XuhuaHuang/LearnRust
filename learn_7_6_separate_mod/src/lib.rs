/** lib.rs
  * separate modules into different files
  *
  * Xuhua Huang
  * April 2021
  */

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}