use std::ops::Mul;

trait Shape<T> {
    fn area(&self) -> T;
}

//T = generic of Multiplication
struct Rectangle<T: Mul> {
    x: T,
    y: T,
}

//Copy trait for Shape T aand implementing for Rectangle T.
// where of our output has to be a type T
//Reason of Copy here is because we're using reference self.x and self.y , otherwise it will not compile
impl<T: Copy> Shape<T> for Rectangle<T> where T: Mul<Output=T>, {
    fn area(&self) -> T {
        self.x * self.y
    }
}

fn main() {
    let r = Rectangle { x: 10, y: 20 };
    let r2 = Rectangle { x: 10.10, y: 20.31 };

    println!("{} {} ", r.area(), r2.area());
}
