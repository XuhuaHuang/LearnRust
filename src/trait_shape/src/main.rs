trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    length: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.height)
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

fn main() {
    let rectangle = Rectangle {
        length: 2.0,
        height: 3.0,
    };
    let circle = Circle { radius: 1.0 };

    println!("Rectangle area: {}", rectangle.area());
    println!("Rectangle perimeter: {}", rectangle.perimeter());

    println!("Circle area: {}", circle.area());
    println!("Circle perimeter: {}", circle.perimeter());
}
