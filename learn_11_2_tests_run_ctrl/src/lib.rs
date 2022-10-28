fn prints_and_returns_10(a: i32) -> i32 {
    println!("function prints_and_returns_10 got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// cargo test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value: i32 = prints_and_returns_10(4);
        dbg!(value);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value: i32 = prints_and_returns_10(8);
        dbg!(value);
        assert_eq!(5, value);
    }

    // cargo test add
    // the "add" will filter out the fn one_hundred()
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
