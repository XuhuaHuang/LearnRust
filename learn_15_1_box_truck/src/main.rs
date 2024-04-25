/**
 * Box<T> is a smart pointer that implements the Deref trait, which allows Box<T> to be treated like a reference.
 */

trait Vehicle {
    fn drive(&self);
}

struct Truck;

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck driving");
    }
}

fn main() {
    // dyn stands for dynamic, such keyword is required to allocate object
    // dynamically on the heap
    let t: Box<dyn Vehicle>;
    t = Box::new(Truck);
    t.drive();
}
