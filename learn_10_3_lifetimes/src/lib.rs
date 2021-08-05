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
