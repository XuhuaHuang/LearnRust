fn main() {

    #[derive(Debug)]
    struct GenericRectangle<T, U> {
        length: T,
        height: U,
    }

    let rec: GenericRectangle<i32, f64> = GenericRectangle {
        length: 4,
        height: 10.5,
    };
    assert!(rec.length == 4);
    assert!(rec.height == 10.5);
    println!("{:#?}", rec);

    #[derive(Debug)]
    struct Rectangle {
        lengh: f32,
        width: f32,
    }

    trait Shape {
        fn new(lengh: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    impl Shape for Rectangle {
        fn new(lengh: f32, width: f32) -> Rectangle {
            Rectangle { lengh, width }
        }

        fn area(&self) -> f32 {
            self.lengh * self.width
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    assert!(rec.area() == 100.0);
    println!("{:#?}", rec);
}
