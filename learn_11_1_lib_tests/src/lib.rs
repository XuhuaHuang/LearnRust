/**
 * Test module and function automatically generated by cargo new
 * To run this test:
 * $ cargo test
 */


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result: u8 = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test() {
        // uncomment the following line to see a failed test
        // panic!("This test is expected to fail");
        assert_eq!(2 * 2, 4);
    }
}

/**
 * Create a custom struct
 * with debug information and define the struct in the implementation block
 */
#[derive(Debug)]
#[derive(PartialEq)] // provide overloaded operator==
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block of struct Rectangle
impl Rectangle {

    // area() borrows self immutably
    // dies NOT take ownership
    // &self == rect: &Rectangle since declared "implementing Rectangle"
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // can_hold() implementation
    // immutable borrow of self and other Rectangle
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height // AND operator of type boolean
    }

    // without the &self keyword making it a static function
    // commonly-used in factory design pattern
    pub fn create_square(length: u32) -> Rectangle {
        Rectangle {
            width: length,
            height: length,
        }
    }

}

#[cfg(test)]
mod rectangle_tests {
    use super::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger: Rectangle = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller: Rectangle = Rectangle {
            width: 6, 
            height: 5,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn area_correct() {
        let test: Rectangle = Rectangle {
            width: 4,
            height: 5,
        };
        assert_eq!(test.area(), 20);
    }
    
    /* Syntax for custom failure message */
    #[test]
    fn square_correct() {
        let test: Rectangle = Rectangle {
            width: 9,
            height: 9
        };
        let returned: Rectangle = Rectangle::create_square(9);
        assert_eq!(test, returned);
    }
}
