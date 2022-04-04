/**
 * guess_test_lib.rs
 * Implement a Guess struct and provide unit tests
 * $ cargo build
 * $ cargo test
 * $ cargo run
 */

#[derive(PartialEq)] // provide overloaded operator==
pub struct Guess {
    value: u8,
    valid: bool,
}

impl Guess {

    /* Factory method */
    pub fn new(value: u8) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        // program would have aborted if the value were invalid
        Guess { value, valid: true }
    } // end Guess::new()

    /* Accessors */
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn valid(&self) -> bool {
        self.valid
    }

} // end implementation block

#[cfg(test)]
mod guess_struct_unit_tests {
    use super::Guess;

    #[test]
    #[should_panic]
    fn guess_greater_than_100() {
        Guess::new(100);
    }

    #[test]
    fn using_result() -> Result<bool, String> {
        if Guess::new(2) == Guess::new(2) {
            Ok(true);
        } else {
            Err(String::from("Factory function behaves differently than expected."))
        }
    }
}