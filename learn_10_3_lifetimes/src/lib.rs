/** lib.rs
* separate file holding some basic function definitions
*
* Xuhua Huang
* Created: Aug 3, 2021
* Last updated: Aug 3, 2021
 */

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/* Demonstration of Specifying Generic Type Parameters,
  * trait bounds,
  * and lifetimes
  */
use std::fmt::Display;
pub fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}